use std::time::Duration;

use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, KeyCode, KeyEvent},
    prelude::CrosstermBackend,
    widgets::Block,
};

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.run(&mut terminal)?;
    ratatui::restore();
    return Ok(());
}

struct App {
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false }
    }

    pub fn run<T: std::io::Write>(
        &mut self,
        term: &mut Terminal<CrosstermBackend<T>>,
    ) -> Result<(), std::io::Error> {
        while !self.exit {
            self.handle_events()?;
            term.draw(|frame| self.render(frame))?;
        }

        return Ok(());
    }

    pub fn render(&self, frame: &mut Frame) {
        frame.render_widget(Block::default(), frame.area());
    }

    pub fn handle_keys(&mut self, ke: KeyEvent) -> Result<(), std::io::Error> {
        match ke.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        };
        return Ok(());
    }

    pub fn handle_events(&mut self) -> Result<(), std::io::Error> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(ke) => self.handle_keys(ke)?,
                _ => {}
            };
        }
        return Ok(());
    }
}
