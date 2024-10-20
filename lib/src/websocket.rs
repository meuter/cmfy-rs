use crate::Result;
use futures_util::StreamExt;
use http::Uri;
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio_websockets::{MaybeTlsStream, WebSocketStream};

pub struct MessageStream {
    websocket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl MessageStream {
    pub fn new(websocket: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        Self { websocket }
    }

    pub async fn open(address: impl AsRef<str>) -> Result<Self> {
        let uri = address.as_ref().parse::<Uri>()?;
        let (stream, _) = tokio_websockets::ClientBuilder::from_uri(uri).connect().await?;
        Ok(MessageStream::new(stream))
    }

    pub async fn next_json<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        while let Some(message) = self.websocket.next().await.transpose()? {
            if let Some(text) = message.as_text() {
                let value = serde_json::from_str(text)?;
                return Ok(Some(value));
            }
        }
        Ok(None)
    }
}
