use clap::Args;
use cmfy::{Client, Result};

use super::Run;

/// Cancel currently running prompt
#[derive(Debug, Args)]
pub struct Cancel;

impl Run for Cancel {
    async fn run(self, client: Client) -> Result<()> {
        client.cancel_running_prompt().await
    }
}
