use asciigraphix_core::shapes::Point;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event as CrosstermEvent, KeyCode},
};

use crate::{
    event::{AppEvent, BeszelEvent, Event, EventHandler, HandleEvent, NtfyEvent, PiholeEvent},
    integrations::{beszel::BeszelHandler, ntfy::NtfyHandler, pihole::PiholeHandler},
    ui::{self, UI},
    utils::InsideRect,
};

pub struct App {
    exit: bool,
    pub event_handler: EventHandler,
    pub beszel_handler: Option<BeszelHandler>,
    pub ntfy_handler: Option<NtfyHandler>,
    pub pihole_handler: Option<PiholeHandler>,
    pub ui: UI,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_handler: EventHandler::new(),
            beszel_handler: None,
            ntfy_handler: None,
            pihole_handler: None,
            ui: UI::default(),
        }
    }

    pub async fn run(mut self, mut term: DefaultTerminal) -> Result<()> {
        self.event_handler
            .send(AppEvent::Beszel(BeszelEvent::Initialize))?;
        self.event_handler
            .send(AppEvent::Ntfy(NtfyEvent::Initialize))?;
        self.event_handler
            .send(AppEvent::Pihole(PiholeEvent::Initialize))?;

        while !self.exit {
            term.draw(|frame| ui::UI::render(&mut self, frame))?;
            self.handle_events().await?;
        }

        Ok(())
    }

    fn handle_crossterm(&mut self, ce: CrosstermEvent) -> Result<()> {
        match ce {
            CrosstermEvent::Key(ke) => match ke.code {
                KeyCode::Char('q') => {
                    self.event_handler
                        .send(AppEvent::Pihole(PiholeEvent::Logout))?;
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
        Ok(())
    }

    fn handle_click(&mut self, c: u16, r: u16) {
        if self.ui.message.is_some() {
            self.ui.message = None;
            return;
        }

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
        if self.ui.pihole_area.inside(c as usize, r as usize) {
            self.ui.pihole_state.borrow_mut().click(&self);
        }
    }

    async fn handle_events(&mut self) -> Result<()> {
        match self.event_handler.next().await? {
            Event::Crossterm(ke) => {
                self.handle_crossterm(ke)?;
            }
            Event::App(AppEvent::ClearMsg) => {
                self.ui.message = None;
            }
            Event::App(AppEvent::Beszel(event)) => {
                BeszelHandler::handle_event(self, event).await?;
            }
            Event::App(AppEvent::Ntfy(event)) => {
                NtfyHandler::handle_event(self, event).await?;
            }
            Event::App(AppEvent::Pihole(event)) => {
                PiholeHandler::handle_event(self, event).await?;
            }
            Event::Tick => {
                self.ui
                    .tubu_state
                    .borrow_mut()
                    .shape
                    .rotate(&Point(0.0, 0.0, 0.0), (0.0, 0.02, 0.1));
            }
        };
        Ok(())
    }
}
