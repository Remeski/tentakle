use std::time::Duration;

use ratatui::layout::{Constraint, Flex, Layout, Margin, Size};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Paragraph, StatefulWidget, Widget};
use tokio::time::{self, Instant};

use crate::integrations::beszel::records::List;
use crate::integrations::beszel::records::System;
use crate::utils::Grid;

pub struct Systems {
    systems: Vec<SystemStatus>,
}

pub struct SystemsState {
    blink: bool,
    timer: time::Instant,
}

struct SystemStatus {
    name: String,
    online: bool,
}

impl Systems {
    pub fn new(systems: &List<System>) -> Self {
        Self {
            systems: Self::process_systems(systems),
        }
    }

    fn process_systems(systems: &List<System>) -> Vec<SystemStatus> {
        systems
            .items
            .iter()
            .map(|item| SystemStatus {
                name: item.name.clone(),
                online: item.status == "up",
            })
            .collect()
    }
}

impl Default for SystemsState {
    fn default() -> Self {
        SystemsState {
            blink: false,
            timer: Instant::now()
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
        if state.timer.elapsed() > Duration::from_secs_f32(0.5) {
            state.blink = !state.blink;
            state.timer = Instant::now();
        }

        let num_systems = self.systems.len();
        let num_rows = num_systems / 4 + if num_systems % 4 != 0 { 1 } else { 0 };
        let area = area.resize(Size::new(area.width, (num_rows + 2) as u16));

        Block::bordered()
            .title("Hosts".fg(Color::Rgb(255, 255, 255)))
            .style(Style::new().fg(Color::Rgb(30, 30, 30)))
            .render(area, buf);
        // let layout = Layout::vertical(vec![Constraint::Length(1); self.systems.len()]).split(area);

        Block::new()
            .style(Style::new().fg(Color::Rgb(255, 255, 255)))
            .render(area.inner(Margin::new(1, 1)), buf);

        let layout = Grid::new(
            area.inner(Margin::new(1, 1)),
            vec![Constraint::Percentage(25); 4],
            vec![Constraint::Length(1); num_rows],
        );

        for (i, area) in layout.single_dimensional().iter().enumerate() {
            let s = self.systems.get(i);
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
                Paragraph::new(char).style(Style::new().fg(Color::Rgb(20, 100, 0)))
            } else {
                Paragraph::new(char).style(Style::new().fg(Color::Rgb(200, 0, 0)))
            };
            name.render(inner[1], buf);
            status.render(inner[0], buf);
        }
    }
}
