use super::Run;
use clap::Args;

use cmfy::{Client, PromptAndStatus, Result, Status};
use colored::Colorize;
use std::iter::empty;

/// List all prompts from history and queue
#[derive(Debug, Args, Default)]
pub struct List {
    /// Display prompts from history
    #[clap(short = 's', long, action, default_value_t = false)]
    pub history: bool,

    /// Display prompts from queue
    #[clap(short, long, action, default_value_t = false)]
    pub queue: bool,

    /// Display all prompts from history and queue
    #[clap(short, long, action, default_value_t = true)]
    pub all: bool,

    /// Display URLs of output image for completed prompts
    #[clap(short, long, action, default_value_t = false)]
    pub images: bool,
}

impl List {
    pub fn history() -> Self {
        Self {
            history: true,
            ..Self::default()
        }
    }

    pub fn queue() -> Self {
        Self {
            queue: true,
            ..Self::default()
        }
    }

    // TODO: consider moving this to the client struct?
    pub async fn collect_entries(client: &Client, history: bool, queue: bool) -> Result<Vec<PromptAndStatus>> {
        let mut entries = vec![];
        if history {
            let history = client.history().await?;
            entries.extend(history.into_iter().map(|entry| {
                if entry.cancelled() {
                    PromptAndStatus::cancelled(entry.prompt)
                } else {
                    PromptAndStatus::completed(entry.prompt, entry.outputs)
                }
            }))
        }
        if queue {
            let queue = client.queue().await?;
            entries.extend(
                empty()
                    .chain(queue.running.into_iter().map(PromptAndStatus::running))
                    .chain(queue.pending.into_iter().map(PromptAndStatus::pending)),
            );
        }
        entries.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
        Ok(entries)
    }
}

impl Run for List {
    async fn run(mut self, client: cmfy::Client) -> Result<()> {
        if self.history || self.queue {
            self.all = false;
        }
        if self.all {
            self.history = true;
            self.queue = true;
        }

        for entry in Self::collect_entries(&client, self.history, self.queue).await? {
            let prompt = entry.prompt;
            let index = format!("[{}]", prompt.index.to_string().bright_blue());
            print!("{:<15}{} ({})", index, prompt.uuid, entry.status);
            if self.images {
                if let Status::Completed(outputs) = entry.status {
                    if let Some(image) = outputs.images().next() {
                        let url = client.url_for_image(image)?.to_string();
                        print!(" -> {}", url.cyan().underline());
                    }
                }
            }
            println!();
        }
        Ok(())
    }
}
