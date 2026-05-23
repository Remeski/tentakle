use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};

use crate::integrations::beszel::records::List;
use crate::integrations::beszel::records::System;

pub struct Systems {
    systems: Vec<SystemStatus>,
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

impl Widget for Systems {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical(vec![Constraint::Length(1); self.systems.len()]).split(area);
        for (i, area) in layout.iter().enumerate() {
            let s = self.systems.get(i).expect("index out of bounds");
            let inner = Layout::horizontal([Constraint::Percentage(5), Constraint::Percentage(95)])
                .flex(Flex::SpaceEvenly)
                .split(*area);
            let name = Paragraph::new(s.name.clone());
            let status = if s.online {
                Paragraph::new("●").style(Style::new().fg(Color::Rgb(20, 100, 0)))
            } else {
                Paragraph::new("○").style(Style::new().fg(Color::Rgb(200, 0, 0)))
            };
            name.render(inner[1], buf);
            status.render(inner[0], buf);
        }
    }
}
