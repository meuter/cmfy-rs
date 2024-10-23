use super::{List, Run};
use clap::Args;
use cmfy::Client;

/// Lists and/or clears history
#[derive(Debug, Args)]
pub struct History {
    /// Lists all prompt from history
    #[clap(long, short, action, default_value_t = false)]
    list: bool,

    /// Clears all prompt from history
    #[clap(long, short, action, default_value_t = false)]
    clear: bool,
}

impl Run for History {
    async fn run(self, client: Client) -> cmfy::Result<()> {
        if self.list {
            List::history().run(client.clone()).await?
        }
        if self.clear {
            client.clear_history().await?;
        }
        Ok(())
    }
}
