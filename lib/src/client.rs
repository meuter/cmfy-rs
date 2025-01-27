use crate::{
    dto::{self, PromptBatch},
    error::Result,
    websocket::MessageStream,
};
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::Client,
    pub hostname: String,
    pub port: u32,
    pub id: String,
}

impl Client {
    pub fn new(hostname: impl AsRef<str>, port: u32, id: String) -> Self {
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

    pub fn url_for_image(&self, image: &dto::Image) -> Url {
        let params = serde_urlencoded::to_string(image).unwrap();
        let address = format!("http://{}:{}/api/view?{}", self.hostname, self.port, params);
        Url::parse(address.as_str()).unwrap()
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

    pub async fn delete_from_history(&self, prompt_id: impl AsRef<str>) -> Result<()> {
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
        let payload = serde_json::json!({
            "client_id": client_id,
            "prompt": nodes
        });
        let response = self.post("prompt", &payload).await?;
        response.ok_or("invalid response".into())
    }

    pub async fn collect_prompt_batch(&self, history: bool, queue: bool) -> Result<PromptBatch> {
        let mut batch = vec![];
        if history {
            let history = self.history().await?;
            batch.extend(history.into_batch_entries())
        }
        if queue {
            let queue = self.queue().await?;
            batch.extend(queue.into_batch_entries())
        }
        batch.sort_by(|l, r| l.inner.index.cmp(&r.inner.index));
        Ok(batch)
    }

    pub async fn listen(&self) -> Result<MessageStream> {
        let address = format!("ws://{}:{}/ws?clientId={}", self.hostname, self.port, self.id);
        MessageStream::open(address).await
    }
}
