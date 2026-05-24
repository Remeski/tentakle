use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event as CrosstermEvent, KeyCode},
};

use crate::{
    event::{AppEvent, Event, EventHandler},
    integrations::beszel::BeszelHandler,
    ui::{self, UI},
    utils::InsideRect,
};

pub struct App {
    exit: bool,
    event_handler: EventHandler,
    pub beszel_handler: Option<BeszelHandler>,
    pub ui: UI,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_handler: EventHandler::new(),
            beszel_handler: None,
            ui: UI::default(),
        }
    }

    pub async fn run(mut self, mut term: DefaultTerminal) -> Result<()> {
        self.event_handler.send(AppEvent::BeszelInitialize).await?;

        while !self.exit {
            term.draw(|frame| ui::UI::render(&mut self, frame))?;
            self.handle_events().await?;
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
            self.ui.systems_state.borrow_mut().next_page();
        }
        if self.ui.containers_area.inside(c as usize, r as usize) {
            if !self.ui.focus_containers() {
                self.ui.containers_state.borrow_mut().next_page();
            }
        }
        if self.ui.las_area.inside(c as usize, r as usize) {
            if !self.ui.focus_las() {
                self.ui.las_state.borrow_mut().next_host();
            }
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
                } else {
                    self.event_handler.send(AppEvent::BeszelInitialize).await?;
                }
            }
            Event::App(AppEvent::BeszelInitialize) => {
                let beszel_handler = BeszelHandler::new().await;
                if !beszel_handler.is_err() {
                    self.beszel_handler = Some(beszel_handler.unwrap());
                    self.event_handler.send(AppEvent::BeszelUpdate).await?;
                }
            }
            Event::App(AppEvent::BeszelChangeLoadAverageHost) => {
                self.ui.las_state.borrow_mut().next_host();
            }
            _ => {}
        };
        return Ok(());
    }
}
