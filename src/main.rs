use std::error::Error;

use reqwest::Url;

use crate::tui::App;

mod integrations;
mod tui;

use integrations::beszel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut beszel = beszel::Client::new(Url::parse("https://beszel.etremes.net")?);
    beszel
        .connect_auth_password(
            String::from("admin@etremes.net"),
            String::from("DL34VzyXxyBZvejT"),
        )
        .await?;

    let system = beszel.system("ebgqyz4q3sn1bx3").await?;
    dbg!(system);

    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.run(&mut terminal)?;

    ratatui::restore();
    return Ok(());
}
