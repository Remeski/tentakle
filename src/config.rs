use std::fs;

use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub beszel: Beszel,
}

#[derive(Deserialize)]
pub struct Beszel {
    pub url: String,
    pub identity: String,
    pub password: String,
}

pub fn read_config() -> Result<Config> {
    let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;
    Ok(config)
}
