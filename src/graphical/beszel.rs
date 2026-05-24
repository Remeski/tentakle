use std::collections::HashMap;
use std::time::Duration;

use ratatui::layout::{Constraint, Flex, Layout, Margin, Rect};
use ratatui::style::{Style, Styled, Stylize};
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Widget};
use tokio::time::{self, Instant};

use crate::graphical::{PagedBlock, colors};
use crate::integrations::beszel::records::System;
use crate::integrations::beszel::records::{Container, List};
use crate::utils::Grid;

pub struct Systems {
    systems: Vec<SystemStatus>,
}

pub struct SystemsState {
    blink: bool,
    blink_timer: time::Instant,
    page: usize,
    num_pages: usize
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
            num_pages: 0
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
        let num_rows = num_systems / num_cols + if num_systems % 4 != 0 { 1 } else { 0 };

        let area = area.clamp(Rect::new(area.left(), area.top(), area.width, (num_rows + 2) as u16));
        let area_inner = area.inner(Margin::new(1, 1));

        let num_pages = num_rows as u16 / area_inner.height + if num_rows as u16 % area_inner.height != 0 { 1 } else { 0 };
        state.num_pages = num_pages as usize;

        PagedBlock::new("Hosts", state.page, state.num_pages).render(area, buf);

        let layout = Grid::new(
            area_inner,
            vec![Constraint::Percentage(25); num_cols],
            vec![Constraint::Length(1); num_rows],
        );

        for (i, area) in layout.single_dimensional().iter().enumerate() {
            let s = self.systems.get(num_cols*(area_inner.height as usize)*state.page + i);
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

impl<'a> Containers<'a> {
    pub fn new(containers: &'a HashMap<String, Vec<Container>>, order: &'a Vec<String>) -> Self {
        Self { containers, order }
    }
}

impl<'a> Widget for Containers<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let outer_block = Block::bordered()
            .title("Containers".fg(colors::FG_TEXT))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::new().fg(colors::BORDER));

        outer_block.render(area, buf);
        let inner_area = area.inner(Margin::new(1, 1));

        let grid = Grid::new(
            inner_area,
            vec![Constraint::Percentage(25); 4],
            vec![Constraint::Length(1); inner_area.height as usize],
        );

        // let container_names = self.containers.keys();

        grid.cols().iter().enumerate().for_each(|(i, col)| {
            let container_name = self.order.get(i);
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
                    if i == remaining_len - 1 && containers_len > remaining_len  {
                        let n_hidden = containers.len() as isize - i as isize;
                        Paragraph::new(
                            format!("{} hidden", n_hidden).fg(colors::BORDER)
                        )
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

        // self.order.into_iter().enumerate().for_each(|(i, name)| {
        //     if let Some(col) = grid.cols().get(i) {
        //         Paragraph::new(format!("{}", name.clone()).set_style(Style::new().bold()))
        //             .render(*col.get(0).unwrap(), buf);
        //
        //         let containers = self.containers.get(name).unwrap();
        //
        //         containers.iter().enumerate().for_each(|(j, container)| {
        //             if j == 0 {
        //                 return;
        //             }
        //             if let Some(rect) = col.get(j) {
        //                 let symbol_color = match container.health {
        //                     0 => colors::SUCCESS,
        //                     2 => colors::SUCCESS,
        //                     3 => colors::ERROR,
        //                     _ => colors::BORDER,
        //                 };
        //                 let symbol = match container.health {
        //                     0 => "○",
        //                     2 => "●",
        //                     3 => "●",
        //                     _ => "○",
        //                 };
        //                 // Paragraph::new(format!("{} {}", symbol.fg(colors::SUCCESS), container.name.clone())).render(*rect, buf);
        //
        //                 let inner = Layout::horizontal([
        //                     Constraint::Percentage(10),
        //                     Constraint::Percentage(90),
        //                 ])
        //                 .flex(Flex::SpaceBetween)
        //                 .split(*rect);
        //                 let name = Paragraph::new(container.name.clone());
        //                 let status = Paragraph::new(symbol).style(Style::new().fg(symbol_color));
        //                 name.render(inner[1], buf);
        //                 status.render(inner[0], buf);
        //             }
        //         });
        //     }
        // });
    }
}
