use std::str::FromStr;

use futures::StreamExt;
use reqwest::Url;
use serde::Deserialize;
use tokio::{select, sync::mpsc::UnboundedSender};
use tokio_tungstenite::connect_async;

use crate::config::Config;
use crate::event::{AppEvent, Event, HandleEvent, NtfyEvent};
use crate::ui::Message;

pub struct NtfyHandler {
    sender: UnboundedSender<Event>,
    url: Url,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NtfyMessage {
    pub id: String,
    pub sequence_id: Option<String>,
    pub time: usize,
    pub expires: Option<usize>,
    pub event: String,
    pub topic: String,
    pub message: Option<String>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl NtfyHandler {
    pub fn new(config: &Config, sender: UnboundedSender<Event>) -> Self {
        let url = Url::from_str(&config.ntfy.url).expect("unable to get url from string");
        Self { sender, url }
    }

    pub async fn init(&self) {
        let connection = connect_async(self.url.as_str()).await;

        if let Ok(connection) = connection {
            let (ws_stream, _) = connection;
            let (_, mut read) = ws_stream.split();

            let sender = self.sender.clone();
            tokio::spawn(async move {
                loop {
                    select! {
                        socket = read.next() => {
                            if let Some(data) = socket {
                                if let Ok(data) = data {
                                    let msg = String::from(data.to_text().expect("not text"));
                                    tracing::trace!(msg = ?msg, "received message from Ntfy");
                                    let ntfy = serde_json::from_str::<NtfyMessage>(&msg);
                                    if let Ok(ntfy) = ntfy {
                                        sender.send(Event::App(AppEvent::Ntfy(NtfyEvent::Msg(ntfy)))).unwrap_or_else(|err| { tracing::error!(err = ?err, "unable to send NtfyMessage")});
                                    }
                                } else {
                                    tracing::error!(err = ?data.unwrap_err(), "unable to process ntfy");
                                }
                            }
                        }
                    }
                }
            });
        } else {
            tracing::error!(err = ?connection.unwrap_err(), "unable to connect to ntfy");
        }
    }
}

impl HandleEvent for NtfyHandler {
    type Event = NtfyEvent;
    async fn handle_event(
        app: &mut crate::app::App,
        event: Self::Event,
    ) -> color_eyre::eyre::Result<()> {
        match event {
            NtfyEvent::Initialize => {
                let handler = NtfyHandler::new(&app.config, app.event_handler.sender.clone());
                handler.init().await;
                app.ntfy_handler = Some(handler);
            }
            NtfyEvent::Msg(msg) => {
                if msg.message.is_some() {
                    app.ui.message = Some(Message {
                        title: msg.title.unwrap_or("".to_string()),
                        content: msg.message.unwrap(),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }
}
