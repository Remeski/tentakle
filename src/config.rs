use std::collections::HashMap;

use clap::Parser;

use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub beszel: Beszel,
    pub pihole: Pihole,
    pub ntfy: Ntfy,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Beszel {
    pub url: String,
    pub identity: String,
    pub password: String,
    pub poll_interval: Option<usize>,
    pub load_averages_order: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Pihole {
    pub url: String,
    pub password: String,
    pub poll_interval: Option<usize>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ntfy {
    pub url: String,
}

#[derive(Deserialize, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("tentakle.toml"))]
    config_path: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_cli = Cli::parse();
        let config_path = config_cli.config_path;
        let settings = config::Config::builder()
            .add_source(config::File::with_name(&config_path).required(false))
            .add_source(config::Environment::with_prefix("tkle").separator("_"))
            .build()?;

        let cfg: Config = settings.try_deserialize()?;

        Ok(cfg)
    }
}
