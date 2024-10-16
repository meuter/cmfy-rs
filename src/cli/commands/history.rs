use super::{list::PromptList, Run};
use clap::Args;

/// Lists prompts from history
#[derive(Debug, Args)]
pub struct History {
    /// Clears all prompt from history after printing
    #[clap(long, short, action, default_value_t = false)]
    clear: bool,
}

impl Run for History {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let history = client.history().await?;
        PromptList::from(history).display();
        if self.clear {
            let payload = serde_json::json!({"clear":true});
            client.post("history", &payload).await?;
        }
        Ok(())
    }
}
