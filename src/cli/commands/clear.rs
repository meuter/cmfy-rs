use clap::Args;
use cmfy::{Client, Result};

use super::Run;

/// clear currently running prompt
#[derive(Debug, Args)]
pub struct Clear;

impl Run for Clear {
    async fn run(self, client: Client) -> Result<()> {
        client.clear_queue().await?;
        client.clear_history().await?;
        client.cancel_running_prompt().await?;
        Ok(())
    }
}
