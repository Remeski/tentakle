use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use chrono::Utc;
use color_eyre::Result;
use reqwest::Url;
use tracing::error;

use crate::{
    config::{self, read_config},
    event::{AppEvent, BeszelEvent, Event, HandleEvent},
    graphical::{self},
    integrations::beszel::{
        api::Client,
        records::{Container, List, System},
    },
};

pub mod api;
pub mod records;

#[derive(Debug)]
pub struct BeszelHandler {
    client: Client,
    systems: Option<List<System>>,
    containers: Option<Containers>,
    load_averages: Option<LoadAverages>,
}

#[derive(Debug)]
struct Containers {
    containers: HashMap<String, Vec<Container>>,
    order: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LoadAverages {
    pub order: Vec<String>,
    pub las: HashMap<String, Vec<LoadAverage>>,
}

/// la: [one minute, five minute, fifteen minute]
#[derive(Debug, Clone)]
pub struct LoadAverage {
    pub la: [f64; 3],
    pub t: chrono::DateTime<Utc>,
}

impl BeszelHandler {
    pub async fn new() -> Result<Self> {
        let config = config::read_config();
        let mut client = Client::new(Url::parse(&config.beszel.url)?);
        client
            .connect_auth_password(config.beszel.identity, config.beszel.password)
            .await?;

        Ok(Self {
            client,
            systems: None,
            containers: None,
            load_averages: None,
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
        tracing::trace!("updating Beszel information");

        let systems = self.client.systems().await;

        if let Ok(systems) = systems {
            self.systems = Some(systems.clone());

            let order = if let Some(order) = read_config().beszel.load_averages_order {
                order
            } else {
                self.systems_to_order(systems)
            };

            let las = self.load_averages(order.clone()).await;

            if let Ok(las) = las {
                let las: LoadAverages = LoadAverages { order, las };
                self.load_averages = Some(las);
            } else {
                self.load_averages = None;
            }
        } else {
            self.systems = None;
            self.load_averages = None;
        }

        let containers = self.client.containers_all().await;

        if let Ok(containers) = containers {
            let mut hm_containers: HashMap<String, Vec<Container>> = HashMap::new();
            let mut containers_order: Vec<String> = Vec::new();
            for cont in containers.items {
                let cont_name = self.system_id_to_name(cont.system.clone());
                if let Some(cont_name) = cont_name {
                    if (chrono::Utc::now() - cont.updated).num_seconds() < 90 {
                        if !hm_containers.contains_key(&cont_name) {
                            containers_order.push(cont_name.clone());
                            hm_containers.insert(cont_name.clone(), vec![cont]);
                        } else {
                            hm_containers.get_mut(&cont_name).unwrap().push(cont);
                        }
                    }
                }
            }
            self.containers = Some(Containers {
                containers: hm_containers,
                order: containers_order,
            });
        } else {
            self.containers = None
        }
    }

    async fn load_averages(&self, order: Vec<String>) -> Result<HashMap<String, Vec<LoadAverage>>> {
        let mut hm: HashMap<String, Vec<LoadAverage>> = HashMap::new();
        for name in order {
            let stats = self.client.system(&name).await.unwrap();
            let las: Vec<LoadAverage> = stats
                .items
                .iter()
                .map(|item| {
                    let la = serde_json::from_value::<records::Stats>(item.stats.clone())
                        .unwrap()
                        .la;
                    // 2022-01-01 10:00:00.123Z
                    LoadAverage {
                        la,
                        t: chrono::NaiveDateTime::parse_from_str(
                            &item.created.trim(),
                            "%Y-%m-%d %H:%M:%S.%3fZ",
                        )
                        .expect("wrong date format")
                        .and_utc(),
                    }
                })
                .collect();
            hm.insert(name.to_string(), las);
        }
        Ok(hm)
    }

    fn systems_to_order(&self, systems: List<System>) -> Vec<String> {
        let mut hs: HashSet<String> = HashSet::new();
        let mut order: Vec<String> = Vec::new();
        for system in &systems.items {
            if !hs.contains(&system.name) {
                order.push(system.name.clone());
                hs.insert(system.name.clone());
            }
        }
        order
    }

    pub fn load_averages_widget(&self) -> Option<graphical::beszel::LAGraph> {
        if let Some(las) = &self.load_averages {
            Some(graphical::beszel::LAGraph::new(las))
        } else {
            None
        }
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

impl HandleEvent for BeszelHandler {
    type Event = BeszelEvent;
    async fn handle_event(app: &mut crate::app::App, event: Self::Event) -> Result<()> {
        match event {
            BeszelEvent::Update => {
                if let Some(bh) = &mut app.beszel_handler {
                    bh.update().await;
                }
            }
            BeszelEvent::Initialize => {
                let beszel_handler = BeszelHandler::new().await;
                if let Ok(beszel_handler) = beszel_handler {
                    app.beszel_handler = Some(beszel_handler);
                    app.event_handler
                        .send(AppEvent::Beszel(BeszelEvent::Update))?;

                    let sender = app.event_handler.sender.clone();
                    let poll_time = Duration::from_secs(
                        config::read_config().beszel.poll_interval.unwrap_or(15) as u64,
                    );

                    let task = async move {
                        let mut beszel_ticker = tokio::time::interval(poll_time);
                        let mut lasstate_ticker = tokio::time::interval(Duration::from_secs(30));
                        loop {
                            tokio::select! {
                                _ = sender.closed() => {
                                    break;
                                }
                                _ = beszel_ticker.tick() => {
                                    sender.send(Event::App(AppEvent::Beszel(BeszelEvent::Update))).unwrap_or_else(|err| {tracing::error!(error = ?err, "couldn't send AppEvent")});
                                }
                                _ = lasstate_ticker.tick() => {
                                    sender.send(Event::App(AppEvent::Beszel(BeszelEvent::ChangeLoadAverageHost))).unwrap_or_else(|err| {tracing::error!(error = ?err, "couldn't send AppEvent")});
                                }
                            }
                        }
                    };

                    tokio::spawn(task);
                } else {
                    error!(error = ?beszel_handler, "Unable to initialize BeszelHandler")
                }
            }
            BeszelEvent::ChangeLoadAverageHost => {
                app.ui.las_state.borrow_mut().next_host();
            }
        }
        Ok(())
    }
}
