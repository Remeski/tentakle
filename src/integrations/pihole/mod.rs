use std::{str::FromStr, time::Duration};

use color_eyre::eyre::Result;
use futures::TryFutureExt;
use reqwest::{Url, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    config::{self, Pihole, read_config},
    event::{AppEvent, Event, HandleEvent, PiholeEvent},
};

#[derive(Debug)]
pub struct PiholeHandler {
    base_url: Url,
    sid: Option<String>,
    pub blocking: Option<bool>,
}

#[derive(Serialize)]
struct AuthRequest {
    password: String,
}

#[derive(Serialize)]
struct AuthPayload {
    sid: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AuthResponse {
    session: AuthResponseSession,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AuthResponseSession {
    sid: String,
    csrf: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct BlockingResponse {
    blocking: String,
}

impl PiholeHandler {
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url: base_url
                .join("api/")
                .expect("something happened with the urls: pihole"),
            sid: None,
            blocking: None,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let password = read_config()
            .expect("unable to read config")
            .pihole
            .password;
        self.authenticate(password).await?;
        tracing::info!(sid = ?self.sid, "logged in and got sid");
        return Ok(());
    }

    async fn authenticate(&mut self, password: String) -> Result<()> {
        let client = reqwest::Client::new()
            .post(
                self.base_url
                    .join("auth/")
                    .expect("something happened with the urls: pihole"),
            )
            .header(CONTENT_TYPE, "application/json")
            .json::<AuthRequest>(&AuthRequest { password });
        let resp: AuthResponse = client.send().await?.json().await?;
        self.sid = Some(resp.session.sid);

        return Ok(());
    }

    pub async fn is_blocking(&self) -> Result<Option<bool>> {
        tracing::info!(self = ?&self);
        if let Some(sid) = &self.sid {
            let client = reqwest::Client::new()
                .get(self.base_url.join("dns/blocking/").expect("bad urls"))
                .json::<AuthPayload>(&AuthPayload { sid: sid.clone() });
            let resp: BlockingResponse = client.send().await?.json().await?;
            let blocking = resp.blocking == "enabled";
            return Ok(Some(blocking));
        }
        Ok(None)
    }

    pub async fn set_blocking(&self, state: bool, timer: usize) -> Result<()> {
        if let Some(sid) = &self.sid {
            let client = reqwest::Client::new()
                .post(self.base_url.join("dns/blocking").expect("bad urls"))
                .json(&json!({"sid": sid.clone(), "blocking": state, "timer": timer}));
            let resp: Value = client.send().await?.json().await?;
            tracing::info!(resp = ?resp);
        }
        Ok(())
    }

    pub async fn logout(&mut self) -> Result<()> {
        if let Some(sid) = &self.sid {
            let client = reqwest::Client::new()
                .delete(self.base_url.join("auth").expect("bad urls"))
                .json(&json!({"sid": sid.clone()}));
            let resp: Value = client.send().await?.json().await?;
            tracing::info!(resp = ?resp);
        }
        Ok(())
    }
}

impl HandleEvent for PiholeHandler {
    type Event = PiholeEvent;
    async fn handle_event(app: &mut crate::app::App, event: Self::Event) -> Result<()> {
        match event {
            PiholeEvent::Initialize => {
                let url = read_config().expect("unable to read config").pihole.url;
                let mut handler = Self::new(Url::from_str(&url).expect("bad url: pihole"));
                let res = handler.initialize().await;
                if res.is_err() {
                    res.unwrap_or_else(
                        |err| tracing::error!(err = ?err, "unable to initialize pihole"),
                    );
                    app.event_handler
                        .send(AppEvent::Pihole(PiholeEvent::Initialize))
                        .unwrap_or_else(|err| tracing::error!(err= ?err, "couldn't send AppEvent"));
                    return Ok(());
                }
                tracing::trace!("pihole initialized");
                app.pihole_handler = Some(handler);

                let sender = app.event_handler.sender.clone();
                let poll_time = Duration::from_secs(
                    config::read_config()?.pihole.poll_interval.unwrap_or(15) as u64,
                );

                let task = async move {
                    let mut pihole_ticker = tokio::time::interval(poll_time);
                    loop {
                        tokio::select! {
                            _ = sender.closed() => {
                                break;
                            }
                            _ = pihole_ticker.tick() => {
                                sender.send(Event::App(AppEvent::Pihole(PiholeEvent::Update))).unwrap_or_else(|err| {tracing::error!(error = ?err, "couldn't send AppEvent")});
                            }
                        }
                    }
                };

                tokio::spawn(task);
            }
            PiholeEvent::Update => {
                if let Some(handler) = &mut app.pihole_handler {
                    handler.blocking = handler.is_blocking().await?;
                    tracing::trace!(blocking = ?handler.blocking, "pihole update");
                }
            }
            PiholeEvent::BlockingOn => {
                if let Some(handler) = &mut app.pihole_handler {
                    tracing::trace!("Pihole blocking on");
                    handler.set_blocking(true, 300).await?;
                }
            }
            PiholeEvent::BlockingOff => {
                if let Some(handler) = &mut app.pihole_handler {
                    tracing::trace!("Pihole blocking off");
                    handler.set_blocking(false, 300).await?;
                }
            }
            PiholeEvent::Logout => {
                if let Some(handler) = &mut app.pihole_handler {
                    tracing::trace!("Pihole logging out");
                    handler.logout().await?;
                }
            }
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
// use crate::integrations::pihole::PiholeHandler;

// #[tokio::test]
// async fn test_authentication() -> Result<()> {
//     let handler = PiholeHandler::new("https://pihole.etremes.net");
// }
// }
