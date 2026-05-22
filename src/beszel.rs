use core::fmt;
use std::error::Error;

use reqwest::{
    Url,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue},
    multipart::Form,
};
use serde::Deserialize;

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
struct AuthRecord {
    id: String,
    collectionId: String,
    collectionName: String,
    created: String,
    updated: String,
    username: String,
    email: String,
    verified: bool,
    emailVisibility: bool,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
struct SystemRecord {
    collectionId: String,
    collectionName: String,
    id: String,
    name: String,
    status: String,
    host: String,
    port: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
struct AuthResponse {
    token: String,
    record: AuthRecord,
}

#[derive(Debug)]
pub struct System {
    pub status: String,
}

pub struct Client {
    pub jwt: Option<String>,
    base_url: Url,
}

impl Client {
    pub fn new(base_url: Url) -> Self {
        return Self {
            jwt: None,
            base_url: base_url,
        };
    }

    pub async fn connect_auth_password(
        &mut self,
        identity: String,
        password: String,
    ) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new()
            .post(self.base_url.join("api/collections/users/auth-with-password")?);
        let mp = client.multipart(
            Form::new()
                .text("identity", identity)
                .text("password", password),
        );
        let resp = mp.send().await?;
        let js = resp.json::<AuthResponse>().await?;

        self.jwt = Some(js.token);
        Ok(())
    }

    pub async fn system(&self, id: &str) -> Result<System, anyhow::Error> {
        let client = reqwest::Client::new().get(
            self.base_url
                .join(&format!("api/collections/systems/records/{}", id))?,
        );
        if let Some(token) = &self.jwt {
            let js = client
                .header(AUTHORIZATION, token.as_str())
                .send()
                .await?
                .json::<SystemRecord>()
                .await?;
            return Ok(System {
                status: js.status
            });
        } else {
            return Err(anyhow::Error::msg("Unauthorized!"));
        }
    }
}
