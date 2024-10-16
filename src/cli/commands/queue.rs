use super::Run;
use clap::Args;

/// Lists prompts from queue
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
            let payload = serde_json::json!({"clear":true});
            client.post("queue", &payload).await?;
        }
        Ok(())
    }
}
