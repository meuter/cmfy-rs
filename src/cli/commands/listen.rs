use super::Run;
use crate::io::{JsonWrite, Output};
use clap::Args;
use cmfy::{Client, Result};
use futures_util::StreamExt;
use http::Uri;

/// Opens a websocket connection to the server, listens for messages
/// and displays them as JSON on the console.
#[derive(Debug, Clone, Args)]
pub struct Listen {
    /// Pretty prints the JSON output
    #[clap(long, action, default_value_t = false)]
    pretty: bool,
}

impl Run for Listen {
    async fn run(self, client: Client) -> Result<()> {
        let address = format!("ws://{}:{}/ws?clientId={}", client.hostname, client.port, client.id);
        let uri = address.parse::<Uri>()?;
        let (mut server, _) = tokio_websockets::ClientBuilder::from_uri(uri).connect().await?;

        while let Some(Ok(message)) = server.next().await {
            if let Some(text) = message.as_text() {
                let parsed: serde_json::Value = serde_json::from_str(text)?;
                Output::std().write_json(&parsed, self.pretty)?;
                println!();
            }
        }
        Ok(())
    }
}
