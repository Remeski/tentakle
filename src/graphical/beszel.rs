use std::collections::HashMap;
use std::time::Duration;

use chrono::Utc;
use ratatui::layout::{Constraint, Flex, Layout, Margin, Rect};
use ratatui::style::{Style, Styled, Stylize};
use ratatui::widgets::{Axis, Chart, Dataset, Paragraph, StatefulWidget, Widget};
use tokio::time::{self, Instant};

use crate::graphical::{PagedBlock, colors};
use crate::integrations::beszel::records::System;
use crate::integrations::beszel::records::{Container, List};
use crate::integrations::beszel::{LoadAverage, LoadAverages};
use crate::utils::Grid;

pub struct Systems {
    systems: Vec<SystemStatus>,
}

pub struct SystemsState {
    blink: bool,
    blink_timer: time::Instant,
    page: usize,
    num_pages: usize,
}

struct SystemStatus {
    name: String,
    online: bool,
}

impl Systems {
    pub fn new(systems: &List<System>) -> Self {
        let systems = systems
            .items
            .iter()
            .map(|item| SystemStatus {
                name: item.name.clone(),
                online: item.status == "up",
            })
            .collect();
        Self { systems }
    }
}

impl SystemsState {
    pub fn next_page(&mut self) {
        self.page += 1;
        self.page %= self.num_pages;
    }
}

impl Default for SystemsState {
    fn default() -> Self {
        SystemsState {
            blink: false,
            blink_timer: Instant::now(),
            page: 0,
            num_pages: 0,
        }
    }
}

impl StatefulWidget for Systems {
    type State = SystemsState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        if state.blink_timer.elapsed() > Duration::from_secs_f32(0.5) {
            state.blink = !state.blink;
            state.blink_timer = Instant::now();
        }

        let num_cols = 4;
        let num_systems = self.systems.len();
        let num_rows = num_systems.div_ceil(num_cols);

        // let area = area.clamp(Rect::new(area.left(), area.top(), area.width, (num_rows + 2) as u16));
        let area_inner = area.inner(Margin::new(1, 1));

        let num_pages = (num_rows as u16).div_ceil(area_inner.height);
        state.num_pages = num_pages as usize;

        PagedBlock::new("Hosts", state.page, state.num_pages).render(area, buf);

        let layout = Grid::new(
            area_inner,
            vec![Constraint::Ratio(1, num_cols as u32); num_cols],
            vec![Constraint::Length(1); num_rows],
        );

        for (i, area) in layout.single_dimensional().iter().enumerate() {
            let s = self
                .systems
                .get(num_cols * (area_inner.height as usize) * state.page + i);
            if s.is_none() {
                break;
            }
            let s = s.unwrap();

            let inner =
                Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)])
                    .flex(Flex::SpaceBetween)
                    .split(*area);
            let name = Paragraph::new(s.name.clone());
            let char = if state.blink { "●" } else { "○" };
            let status = if s.online {
                Paragraph::new(char).style(Style::new().fg(colors::SUCCESS))
            } else {
                Paragraph::new(char).style(Style::new().fg(colors::ERROR))
            };
            name.render(inner[1], buf);
            status.render(inner[0], buf);
        }
    }
}

pub struct Containers<'a> {
    containers: &'a HashMap<String, Vec<Container>>,
    order: &'a Vec<String>,
}

#[derive(Default)]
pub struct ContainersState {
    page: usize,
    num_pages: usize,
}

impl<'a> Containers<'a> {
    pub fn new(containers: &'a HashMap<String, Vec<Container>>, order: &'a Vec<String>) -> Self {
        Self { containers, order }
    }
}

impl<'a> StatefulWidget for Containers<'a> {
    type State = ContainersState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut ContainersState,
    ) where
        Self: Sized,
    {
        PagedBlock::new("Containers", state.page, state.num_pages).render(area, buf);
        let inner_area = area.inner(Margin::new(1, 1));

        let num_cols = 3;
        let num_hosts = self.order.len();

        state.num_pages = num_hosts.div_ceil(num_cols);

        let grid = Grid::new(
            inner_area,
            vec![Constraint::Ratio(1, num_cols as u32); num_cols],
            vec![Constraint::Length(1); inner_area.height as usize],
        );

        // let container_names = self.containers.keys();

        grid.cols().iter().enumerate().for_each(|(i, col)| {
            let container_name = self.order.get(state.page * num_cols + i);
            if let Some(container_name) = container_name {
                Paragraph::new(
                    format!("{}", container_name.clone()).set_style(Style::new().bold()),
                )
                .render(*col.get(0).unwrap(), buf);
                let containers = self.containers.get(container_name).unwrap();
                let containers_len = containers.len();
                let remaining = col.iter().skip(1);
                let remaining_len = remaining.len();
                for (i, rect) in remaining.enumerate() {
                    if i == remaining_len - 1 && containers_len > remaining_len {
                        let n_hidden = containers.len() as isize - i as isize;
                        Paragraph::new(format!("{} hidden", n_hidden).fg(colors::BORDER))
                            .render(*rect, buf);
                        continue;
                    }
                    let container = containers.get(i);
                    if let Some(container) = container {
                        let symbol_color = match container.health {
                            0 => colors::SUCCESS,
                            2 => colors::SUCCESS,
                            3 => colors::ERROR,
                            _ => colors::BORDER,
                        };
                        let symbol = match container.health {
                            0 => "○",
                            2 => "●",
                            3 => "●",
                            _ => "○",
                        };
                        // Paragraph::new(format!("{} {}", symbol.fg(colors::SUCCESS), container.name.clone())).render(*rect, buf);

                        let inner = Layout::horizontal([
                            Constraint::Percentage(10),
                            Constraint::Percentage(90),
                        ])
                        .flex(Flex::SpaceBetween)
                        .split(*rect);
                        let name = Paragraph::new(container.name.clone());
                        let status = Paragraph::new(symbol).style(Style::new().fg(symbol_color));
                        name.render(inner[1], buf);
                        status.render(inner[0], buf);
                    }
                }
            }
        });
    }
}

impl ContainersState {
    pub fn next_page(&mut self) {
        self.page += 1;
        self.page %= self.num_pages;
    }
}

pub struct LAGraph {
    order: Vec<String>,
    las: HashMap<String, Vec<LoadAverage>>,
}

#[derive(Default)]
pub struct LASState {
    host_index: usize,
    num_hosts: usize,
}

impl LAGraph {
    pub fn new(data: &LoadAverages) -> Self {
        LAGraph {
            order: data.order.clone(),
            las: data.las.clone(),
        }
    }

    fn las_to_data(&self, system: &str, offset: f64) -> ([Vec<(f64, f64)>; 3], f64) {
        let las = self.las.get(system).expect("system not found");
        let current_time = Utc::now();

        let mut las1: Vec<(f64, f64)> = Vec::new();
        let mut las5: Vec<(f64, f64)> = Vec::new();
        let mut las15: Vec<(f64, f64)> = Vec::new();

        let mut max: f64 = 0.;

        las.iter().for_each(|la| {
            let dt = current_time - la.t;
            let dt = -dt.as_seconds_f64() + offset;
            let la1 = la.la[0];
            let la5 = la.la[1];
            let la15 = la.la[2];

            max = max.max(la1).max(la5).max(la15);

            if dt < offset && dt > 0. {
                las1.push((dt, la1));
                las5.push((dt, la5));
                las15.push((dt, la15));
            }
        });

        return ([las1, las5, las15], max);
    }
}

impl StatefulWidget for LAGraph {
    type State = LASState;
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        state.num_hosts = self.order.len();

        PagedBlock::new(
            &format!("Load average - {}", &self.order[state.host_index]),
            state.host_index,
            state.num_hosts,
        )
        .render(area, buf);
        let inner_area = area.inner(Margin::new(1, 1));

        let offset = 1.0 * 60. * 60.;
        let offset_text = "1h";

        let (data, max) = self.las_to_data(&self.order[state.host_index], offset);

        let datasets = vec![
            Dataset::default()
                .name("1")
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().red())
                .data(&data[0]),
            Dataset::default()
                .name("5")
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().cyan())
                .data(&data[1]),
            Dataset::default()
                .name("15")
                .graph_type(ratatui::widgets::GraphType::Line)
                .style(Style::default().green())
                .data(&data[2]),
        ];

        let max_str = max.to_string();
        let t_axis = Axis::default()
            .bounds([0., offset])
            .labels([offset_text, "now"])
            .style(Style::default().fg(colors::BORDER));
        let y_axis = Axis::default()
            .bounds([0., max])
            .labels(["0", &max_str])
            .style(Style::default().fg(colors::BORDER));

        let chart = Chart::new(datasets)
            .x_axis(t_axis)
            .y_axis(y_axis)
            .bg(colors::BG_CONT)
            .hidden_legend_constraints((Constraint::Min(0), Constraint::Ratio(1, 1)));
        chart.render(inner_area, buf);
    }
}

impl LASState {
    pub fn next_host(&mut self) {
        if self.num_hosts != 0 {
            self.host_index += 1;
            self.host_index %= self.num_hosts;
        }
    }
}
