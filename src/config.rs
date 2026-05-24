use std::fs;

use clap::{Parser};

use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::trace_dbg;

#[derive(Deserialize)]
pub struct Config {
    pub beszel: Beszel,
}

#[derive(Deserialize)]
pub struct Beszel {
    pub url: String,
    pub identity: String,
    pub password: String,
    pub poll_interval: Option<usize>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("tentakle.toml"))]
    config: String,
}

pub fn read_config() -> Result<Config> {
    let args = Args::parse();

    trace_dbg!(&args);

    let config: Config = toml::from_str(&fs::read_to_string("config.toml")?)?;
    Ok(config)
}
