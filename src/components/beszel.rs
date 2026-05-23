use color_eyre::eyre::Result;
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};
use reqwest::Url;

use crate::config;
use crate::integrations::beszel::records::List;
use crate::integrations::beszel::{Client, records::System};

pub struct Systems {
    systems: Option<Vec<SystemStatus>>,
}

struct SystemStatus {
    name: String,
    online: bool
}

impl Systems {
    pub async fn new() -> Result<Self> {
        let config = config::read_config()?;
        let mut client = Client::new(Url::parse(&config.beszel.url)?);
        client
            .connect_auth_password(config.beszel.identity, config.beszel.password)
            .await?;

        let systems = client.systems().await?;
        let systems = Self::process_systems(systems);

        Ok(Self {
            systems: Some(systems),
        })
    }

    fn process_systems(systems: List<System>) -> Vec<SystemStatus> {
        systems.items.iter().map(|item| 
            SystemStatus {
                name: item.name.clone(),
                online: item.status == "up"
            }
        ).collect()
    }
}

impl Widget for &Systems {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        if let Some(systems) = &self.systems {
            let layout = Layout::vertical(vec![Constraint::Length(1); systems.len()]).split(area);
            for (i, area) in layout.iter().enumerate() {
                let s = systems.get(i).expect("index out of bounds");
                let inner = Layout::horizontal([Constraint::Percentage(5), Constraint::Percentage(95)]).flex(Flex::SpaceEvenly).split(*area);
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
}
