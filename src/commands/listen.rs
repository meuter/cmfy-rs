use super::Run;
use crate::io::{JsonWrite, Output};
use clap::Args;
use cmfy::{Client, Result};

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
        let mut output = Output::default();
        let mut message_stream = client.listen().await?;

        while let Some(value) = message_stream.next_json::<serde_json::Value>().await? {
            output.write_json(&value, self.pretty)?;
            output.writeln()?;
        }
        Ok(())
    }
}
