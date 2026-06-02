use std::panic;

use color_eyre::eyre::Result;
use crossterm::ExecutableCommand;

use clap::Parser;

use crate::app::App;

mod app;
mod config;
mod event;
mod graphical;
mod integrations;
mod logs;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        std::io::stdout()
            .execute(crossterm::event::DisableMouseCapture)
            .unwrap();
        std::panic::take_hook()(info);
    }));
    color_eyre::install()?;

    config::Args::parse();

    logs::initialize_logging()?;

    std::io::stdout().execute(crossterm::event::EnableMouseCapture)?;
    let terminal = ratatui::init();
    let app = App::new();

    let run = app.run(terminal).await;

    ratatui::restore();
    std::io::stdout().execute(crossterm::event::DisableMouseCapture)?;

    return run;
}
