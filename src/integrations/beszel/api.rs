use color_eyre::{Result, eyre::eyre};
use reqwest::{RequestBuilder, Url, header::AUTHORIZATION, multipart::Form};

use crate::integrations::beszel::records::{self, List, System, SystemStats};

pub struct Client {
    http_handler: HTTPHandler,
}

impl Client {
    pub fn new(base_url: Url) -> Self {
        return Self {
            http_handler: HTTPHandler::new(base_url, None),
        };
    }

    pub async fn connect_auth_password(
        &mut self,
        identity: String,
        password: String,
    ) -> Result<()> {
        self.http_handler.authenticate(identity, password).await?;
        Ok(())
    }

    pub async fn systems(&self) -> Result<List<System>> {
        let rb = self.http_handler.get("collections/systems/records")?;
        let js = rb.send().await?.json::<List<System>>().await?;
        Ok(js)
    }

    pub async fn system(&self, system_name: &str) -> Result<List<SystemStats>> {
        let rb = self
            .http_handler
            .get("collections/system_stats/records")?
            .query(&[(
                "filter",
                format!("(system.name='{}')", system_name).as_str(),
            )]);
        let js = rb.send().await?.json::<List<SystemStats>>().await?;

        return Ok(js);
    }
}

struct HTTPHandler {
    token: Option<String>,
    base_url: Url,
}

#[allow(dead_code)]
impl HTTPHandler {
    fn new(base_url: Url, token: Option<String>) -> Self {
        Self {
            token,
            base_url: base_url.join("api/").expect("URL problem"),
        }
    }

    async fn authenticate(&mut self, identity: String, password: String) -> Result<()> {
        let client = reqwest::Client::new()
            .post(self.base_url.join("collections/users/auth-with-password")?);
        let mp = client.multipart(
            Form::new()
                .text("identity", identity)
                .text("password", password),
        );
        let resp = mp.send().await?;
        let js = resp.json::<records::Auth>().await?;

        self.token = Some(js.token);

        Ok(())
    }

    fn get(&self, path: &str) -> Result<RequestBuilder> {
        let client = reqwest::Client::new().get(self.base_url.join(path)?);
        if let Some(token) = &self.token {
            let rb = client.header(AUTHORIZATION, token.as_str());
            return Ok(rb);
        } else {
            return Err(eyre!("unauthorized! token missing"));
        }
    }

    fn post(&self, path: &str) -> Result<RequestBuilder> {
        let client = reqwest::Client::new().post(self.base_url.join(path)?);
        if let Some(token) = &self.token {
            let rb = client.header(AUTHORIZATION, token.as_str());
            return Ok(rb);
        } else {
            return Err(eyre!("unauthorized! token missing"));
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use reqwest::Url;

    use crate::integrations::beszel::api;

    // This is just for development purposes
    #[tokio::test]
    async fn test_beszel_web() -> Result<()> {
        let mut beszel = api::Client::new(Url::parse("https://beszel.etremes.net")?);
        beszel
            .connect_auth_password(
                String::from("admin@etremes.net"),
                String::from("DL34VzyXxyBZvejT"),
            )
            .await?;

        let systems = beszel.systems().await?;
        dbg!(systems);

        let pihole = beszel.system("HL_PIHOLE").await?;
        dbg!(pihole);

        Ok(())
    }
}
