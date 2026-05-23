use color_eyre::eyre::Result;

use crate::tui::App;

mod integrations;
mod tui;
mod event;

#[tokio::main]
async fn main() -> Result<()> {
    let terminal = ratatui::init();
    let app = App::new();

    app.run(terminal).await?;

    ratatui::restore();
    return Ok(());
}
