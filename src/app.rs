use std::cell::RefCell;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event as CrosstermEvent, KeyCode},
};

use crate::{
    graphical::beszel::SystemsState, event::{AppEvent, Event, EventHandler}, integrations::beszel::BeszelHandler, trace_dbg, ui
};

pub struct App {
    exit: bool,
    event_handler: EventHandler,
    pub beszel_handler: Option<BeszelHandler>,
    pub systems_state: RefCell<SystemsState>
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_handler: EventHandler::new(),
            beszel_handler: None,
            systems_state: RefCell::new(SystemsState::default())
        }
    }

    pub async fn run(mut self, mut term: DefaultTerminal) -> Result<()> {
        let beszel_handler = BeszelHandler::new().await?;
        self.beszel_handler = Some(beszel_handler);

        while !self.exit {
            self.handle_events().await?;
            term.draw(|frame| ui::render(&self, frame))?;
        }

        return Ok(());
    }

    fn handle_crossterm(&mut self, ce: CrosstermEvent) -> Result<()> {
        trace_dbg!(&ce);
        match ce {
            CrosstermEvent::Key(ke) => match ke.code {
                KeyCode::Char('q') => {
                    self.exit = true;
                }
                _ => {}
            },
            CrosstermEvent::Mouse(m) => {
                trace_dbg!(m);
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
            Event::App(AppEvent::BeszelUpdate) => {
                if let Some(bh) = &mut self.beszel_handler {
                    bh.update().await;
                }
            }
            _ => {}
        };
        return Ok(());
    }
}
