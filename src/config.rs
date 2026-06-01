use std::fs;

use clap::Parser;

use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub beszel: Beszel,
    pub pihole: Pihole, 
    pub ntfy: Ntfy
}

#[derive(Deserialize)]
pub struct Beszel {
    pub url: String,
    pub identity: String,
    pub password: String,
    pub poll_interval: Option<usize>,
    pub load_averages_order: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Pihole {
    pub url: String,
    pub password: String,
    pub poll_interval: Option<usize>,
}

#[derive(Deserialize)]
pub struct Ntfy {
    pub url: String
}

// #[derive(Deserialize)]
// pub struct Uptimekuma {
//     pub url: String,
//     pub api_key: String
// }

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("tentakle.toml"))]
    config: String,
}

pub fn read_config() -> Config {
    let args = Args::parse();

    let config = args.config;

    let config: Config = toml::from_str(
        &fs::read_to_string(config.clone())
            .expect(&format!("couldn't find config file at {}", config)),
    ).expect("unable to read config");
    config
}
