use std::collections::HashMap;

use color_eyre::Result;
use reqwest::Url;

use crate::{
    config,
    graphical::{self},
    integrations::beszel::{
        api::Client,
        records::{Container, List, System},
    },
};

pub mod api;
pub mod records;

pub struct BeszelHandler {
    client: Client,
    systems: Option<List<System>>,
    containers: Option<Containers>,
}

struct Containers {
    containers: HashMap<String, Vec<Container>>,
    order: Vec<String>,
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
            containers: None,
        })
    }

    fn system_id_to_name(&self, id: String) -> Option<String> {
        if let Some(systems) = &self.systems {
            let f = systems.items.iter().find(|item| item.id == id);
            if f.is_some() {
                Some(f.unwrap().name.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn update(&mut self) {
        let systems = self.client.systems().await.expect("error fetching systems");
        self.systems = Some(systems);

        let containers = self
            .client
            .containers_all()
            .await
            .expect("error fetching containers");
        let mut hm_containers: HashMap<String, Vec<Container>> = HashMap::new();
        let mut containers_order: Vec<String> = Vec::new();
        for cont in containers.items {
            let cont_name = self.system_id_to_name(cont.system.clone()).unwrap();
            if !hm_containers.contains_key(&cont_name) {
                containers_order.push(cont_name.clone());
                hm_containers.insert(cont_name.clone(), vec![cont]);
            } else {
                hm_containers.get_mut(&cont_name).unwrap().push(cont);
            }
        }

        self.containers = Some(Containers {
            containers: hm_containers,
            order: containers_order,
        });
    }

    pub fn systems_widget(&self) -> Option<graphical::beszel::Systems> {
        if let Some(systems) = &self.systems {
            Some(graphical::beszel::Systems::new(systems))
        } else {
            None
        }
    }

    pub fn containers_widget<'a>(&'a self) -> Option<graphical::beszel::Containers<'a>> {
        if let Some(containers) = &self.containers {
            Some(graphical::beszel::Containers::new(
                &containers.containers,
                &containers.order,
            ))
        } else {
            None
        }
    }
}
