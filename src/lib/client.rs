use serde::{de::DeserializeOwned, Serialize};

use crate::{dto, error::Result};

pub struct Client {
    client: reqwest::Client,
    server: String,
    port: u32,
}

impl Client {
    pub fn new(server: impl AsRef<str>, port: u32) -> Self {
        let server = server.as_ref().to_string();
        let client = reqwest::Client::new();
        Self { client, server, port }
    }

    pub async fn get<R: DeserializeOwned>(&self, route: impl AsRef<str>) -> Result<R> {
        let url = format!("http://{}:{}/{}", self.server, self.port, route.as_ref());
        let response = self.client.get(url).send().await?;
        let body = response.error_for_status()?.bytes().await?;
        let parsed = serde_json::from_slice(&body)?;
        Ok(parsed)
    }

    pub async fn post(&self, route: impl AsRef<str>, payload: &impl Serialize) -> Result<()> {
        let url = format!("http://{}:{}/{}", self.server, self.port, route.as_ref());
        let body = serde_json::to_string(payload)?;
        let response = self.client.post(url).body(body).send().await?;
        response.error_for_status()?;
        Ok(())
    }

    pub async fn system_stats(&self) -> Result<dto::SystemStats> {
        self.get("system_stats").await
    }

    pub async fn history(&self) -> Result<dto::History> {
        self.get("history").await
    }

    pub async fn queue(&self) -> Result<dto::Queue> {
        self.get("queue").await
    }
}
