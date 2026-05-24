use std::cell::RefCell;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event as CrosstermEvent, KeyCode},
};

use crate::{
    event::{AppEvent, Event, EventHandler},
    graphical::beszel::SystemsState,
    integrations::beszel::BeszelHandler,
    ui::{self, UI},
    utils::InsideRect,
};

pub struct App {
    exit: bool,
    event_handler: EventHandler,
    pub beszel_handler: Option<BeszelHandler>,
    pub systems_state: RefCell<SystemsState>,
    pub ui: UI,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_handler: EventHandler::new(),
            beszel_handler: None,
            systems_state: RefCell::new(SystemsState::default()),
            ui: UI::default(),
        }
    }

    pub async fn run(mut self, mut term: DefaultTerminal) -> Result<()> {
        let beszel_handler = BeszelHandler::new().await?;
        self.beszel_handler = Some(beszel_handler);

        while !self.exit {
            self.handle_events().await?;
            term.draw(|frame| ui::UI::render(&mut self, frame))?;
        }

        return Ok(());
    }

    fn handle_crossterm(&mut self, ce: CrosstermEvent) -> Result<()> {
        match ce {
            CrosstermEvent::Key(ke) => match ke.code {
                KeyCode::Char('q') => {
                    self.exit = true;
                }
                _ => {}
            },
            CrosstermEvent::Mouse(me) => {
                let (c, r) = (me.column, me.row);
                match me.kind {
                    crossterm::event::MouseEventKind::Up(_) => {
                        self.handle_click(c, r);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Ok(());
    }

    fn handle_click(&mut self, c: u16, r: u16) {
        if self.ui.systems_area.inside(c as usize, r as usize) {
            self.systems_state.borrow_mut().next_page();
        }
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
