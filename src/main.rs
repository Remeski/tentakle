use color_eyre::eyre::Result;
use crossterm::ExecutableCommand;

use crate::app::App;

mod integrations;
mod app;
mod event;
mod components;
mod ui;
mod logs;
mod config;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    logs::initialize_logging()?;

    std::io::stdout().execute(crossterm::event::EnableMouseCapture)?;

    let terminal = ratatui::init();
    let app = App::new();

    app.run(terminal).await?;

    ratatui::restore();
    return Ok(());
}
