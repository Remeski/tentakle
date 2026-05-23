use color_eyre::Result;
use reqwest::Url;

use crate::{
    components::{self},
    config,
    integrations::beszel::{
        api::Client,
        records::{List, System},
    },
};

pub mod api;
pub mod records;

pub struct BeszelHandler {
    client: Client,
    systems: Option<List<System>>,
}

impl BeszelHandler {
    pub async fn new() -> Result<Self> {
        let config = config::read_config()?;
        let mut client = Client::new(Url::parse(&config.beszel.url)?);
        client
            .connect_auth_password(config.beszel.identity, config.beszel.password)
            .await?;

        Ok(Self {
            client,
            systems: None,
        })
    }

    pub async fn update(&mut self) {
        let systems = self.client.systems().await.expect("error fetching systems");
        self.systems = Some(systems);
    }

    pub fn systems_widget(&self) -> Option<components::beszel::Systems> {
        if let Some(systems) = &self.systems {
            Some(components::beszel::Systems::new(systems))
        } else {
            None
        }
    }
}
