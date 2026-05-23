use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event as CrosstermEvent, KeyCode},
    widgets::{Block, Widget},
};

use crate::event::{Event, EventHandler};

pub struct App {
    exit: bool,
    event_handler: EventHandler,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_handler: EventHandler::new(),
        }
    }

    pub async fn run(mut self, mut term: DefaultTerminal) -> Result<()> {
        while !self.exit {
            self.handle_events().await?;
            term.draw(|frame| frame.render_widget(&self, frame.area()))?;
        }

        return Ok(());
    }

    fn handle_crossterm(&mut self, ce: CrosstermEvent) -> Result<()> {
        match ce {
            CrosstermEvent::Key(ke) => {
                match ke.code {
                    KeyCode::Char('q') => {
                        self.exit = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Ok(());
    }

    async fn handle_events(&mut self) -> Result<()> {
        match self.event_handler.next().await? {
            Event::Crossterm(ke) => {
                self.handle_crossterm(ke)?;
            }
            _ => {}
        };
        return Ok(());
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::new().render(area, buf);
    }
}
