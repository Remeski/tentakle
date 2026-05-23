use color_eyre::eyre::Result;

use crate::tui::App;

mod integrations;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.run(&mut terminal)?;

    ratatui::restore();
    return Ok(());
}
