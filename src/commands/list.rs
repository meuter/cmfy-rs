use super::Run;
use clap::Args;

use cmfy::{Client, Result, Status};
use colored::Colorize;

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
}

impl Run for List {
    async fn run(mut self, client: Client) -> Result<()> {
        if self.history || self.queue {
            self.all = false;
        }
        if self.all {
            self.history = true;
            self.queue = true;
        }

        for entry in client.collect_prompt_batch(self.history, self.queue).await? {
            let prompt = entry.inner;
            let index = format!("[{}]", prompt.index.to_string().bright_blue());
            print!("{:<15}{} ({})", index, prompt.uuid, entry.status.colored());
            if self.images {
                if let Status::Completed(outputs) = entry.status {
                    if let Some(image) = outputs.images().next() {
                        let url = client.url_for_image(image).to_string();
                        print!(" -> {}", url.cyan().underline());
                    }
                }
            }
            println!();
        }
        Ok(())
    }
}
