use crate::{dto, error::Result};
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

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

    pub fn base_url(&self) -> Result<Url> {
        let address = format!("http://{}:{}", self.server, self.port);
        let url = Url::parse(address.as_str())?;
        Ok(url)
    }

    pub fn url_for_image(&self, image: &dto::Image) -> Result<Url> {
        let params = serde_urlencoded::to_string(image)?;
        let address = format!("http://{}:{}/api/view?{}", self.server, self.port, params);
        let url = Url::parse(address.as_str())?;
        Ok(url)
    }

    pub async fn get<R: DeserializeOwned>(&self, route: impl AsRef<str>) -> Result<R> {
        let url = format!("http://{}:{}/{}", self.server, self.port, route.as_ref());
        let response = self.client.get(url).send().await?;
        let body = response.error_for_status()?.bytes().await?;
        let parsed = serde_json::from_slice(&body)?;
        Ok(parsed)
    }

    pub async fn post<R: DeserializeOwned>(&self, route: impl AsRef<str>, payload: &impl Serialize) -> Result<Option<R>> {
        let url = format!("http://{}:{}/{}", self.server, self.port, route.as_ref());
        let body = serde_json::to_string(payload)?;
        let response = self.client.post(url).body(body).send().await?;
        let body = response.error_for_status()?.bytes().await?;
        if body.is_empty() {
            Ok(None)
        } else {
            let parsed = serde_json::from_slice(&body)?;
            Ok(Some(parsed))
        }
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

    async fn clear(&self, route: impl AsRef<str>) -> Result<()> {
        let payload = serde_json::json!({"clear":true});
        let response: Option<()> = self.post(route, &payload).await?;
        assert!(response.is_none());
        Ok(())
    }

    pub async fn clear_queue(&self) -> Result<()> {
        self.clear("queue").await
    }

    pub async fn clear_history(&self) -> Result<()> {
        self.clear("history").await
    }

    pub async fn cancel_running_prompt(&self) -> Result<()> {
        let payload = serde_json::Value::Null;
        let response: Option<()> = self.post("interrupt", &payload).await?;
        assert!(response.is_none());
        Ok(())
    }

    pub async fn submit(&self, nodes: &dto::PromptNodes) -> Result<dto::SubmitResponse> {
        let payload = serde_json::json!({"prompt": nodes});
        let response = self.post("prompt", &payload).await?;
        response.ok_or("invalid response".into())
    }
}
