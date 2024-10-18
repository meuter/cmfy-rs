use crate::{dto, error::Result};
use reqwest::Url;
use ring::digest::{digest, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::Client,
    pub hostname: String,
    pub port: u32,
    pub id: Uuid,
}

impl Client {
    pub fn from_hostname_port(hostname: impl AsRef<str>, port: u32) -> Result<Self> {
        // NOTE: When opening a websocket to the server, we only recieve progress
        //       messages from prompts submitted with the same client id as the one
        //       provided in the post message.
        //       Since this is a CLI app, we need a consistent client id accross
        //       multiple invocation. So we take the full absolute path of the
        //       current executable as a basis to generate a Uuid.
        let full_executable_path = std::env::current_exe()?;
        let hash = digest(&SHA256, full_executable_path.as_os_str().as_encoded_bytes());
        let client_id = Uuid::from_slice(&hash.as_ref()[0..16])?;
        Ok(Self::new(hostname, port, client_id))
    }

    fn new(hostname: impl AsRef<str>, port: u32, id: Uuid) -> Self {
        let server = hostname.as_ref().to_string();
        let client = reqwest::Client::new();
        Self {
            client,
            hostname: server,
            port,
            id,
        }
    }

    pub fn base_url(&self) -> Result<Url> {
        let address = format!("http://{}:{}", self.hostname, self.port);
        let url = Url::parse(address.as_str())?;
        Ok(url)
    }

    pub fn url_for_image(&self, image: &dto::Image) -> Result<Url> {
        let params = serde_urlencoded::to_string(image)?;
        let address = format!("http://{}:{}/api/view?{}", self.hostname, self.port, params);
        let url = Url::parse(address.as_str())?;
        Ok(url)
    }

    pub async fn get<R: DeserializeOwned>(&self, route: impl AsRef<str>) -> Result<R> {
        let url = format!("http://{}:{}/{}", self.hostname, self.port, route.as_ref());
        let response = self.client.get(url).send().await?;
        let body = response.error_for_status()?.bytes().await?;
        let parsed = serde_json::from_slice(&body)?;
        Ok(parsed)
    }

    pub async fn post<R: DeserializeOwned>(&self, route: impl AsRef<str>, payload: &impl Serialize) -> Result<Option<R>> {
        let url = format!("http://{}:{}/{}", self.hostname, self.port, route.as_ref());
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

    pub async fn delete_from_history<'a>(&self, prompt_id: impl AsRef<str>) -> Result<()> {
        let delete = vec![prompt_id.as_ref()];
        let payload = serde_json::json!({"delete": delete});
        let response: Option<()> = self.post("history", &payload).await?;
        assert!(response.is_none());
        Ok(())
    }

    pub async fn cancel_running_prompt(&self) -> Result<()> {
        let payload = serde_json::Value::Null;
        let response: Option<()> = self.post("interrupt", &payload).await?;
        assert!(response.is_none());
        Ok(())
    }

    pub async fn submit(&self, nodes: &dto::PromptNodes) -> Result<dto::SubmitResponse> {
        let client_id = self.id.to_string();
        println!("submitting client_id={}", client_id);
        let payload = serde_json::json!({
            "client_id": client_id,
            "prompt": nodes
        });
        let response = self.post("prompt", &payload).await?;
        response.ok_or("invalid response".into())
    }
}
