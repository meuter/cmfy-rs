use super::Run;
use clap::Args;

/// Lists and optionally clears prompts from queue
#[derive(Debug, Args)]
pub struct Queue {
    /// Clears all pending prompts from the queue after printing
    #[clap(long, short, action, default_value_t = false)]
    clear: bool,
}

impl Run for Queue {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let queue = client.queue().await?;
        super::list::PromptList::from(queue).display();
        if self.clear {
            client.clear_queue().await?;
        }
        Ok(())
    }
}
