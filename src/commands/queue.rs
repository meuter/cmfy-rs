use super::{List, Run};
use clap::Args;

/// Lists and/or clears clear queue
#[derive(Debug, Args)]
pub struct Queue {
    /// Lists all pending prompts from queue
    #[clap(long, short, action, default_value_t = false)]
    list: bool,

    /// Clears all pending prompts from queue
    #[clap(long, short, action, default_value_t = false)]
    clear: bool,
}

impl Run for Queue {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        if self.list {
            List::queue().run(client.clone()).await?;
        }
        if self.clear {
            client.clear_queue().await?;
        }
        Ok(())
    }
}
