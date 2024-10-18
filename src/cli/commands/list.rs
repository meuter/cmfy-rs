use super::Run;
use clap::Args;

use cmfy::{dto::Outputs, Client, Prompt, Result};
use colored::Colorize;
use itertools::Itertools;
use std::{fmt::Display, iter::empty};

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
    pub async fn collect_entries(client: &Client, history: bool, queue: bool) -> Result<Vec<PromptListEntry>> {
        use PromptStatus::*;

        let mut entries = vec![];
        if history {
            let history = client.history().await?;
            entries.extend(history.into_iter().map(|entry| {
                let status = if entry.cancelled() { Cancelled } else { Completed };
                let prompt = entry.prompt;
                let outputs = Some(entry.outputs);
                PromptListEntry { prompt, status, outputs }
            }))
        }
        if queue {
            let queue = client.queue().await?;
            entries.extend(
                empty()
                    .chain(queue.running.into_iter().map(|prompt| PromptListEntry {
                        prompt,
                        status: PromptStatus::Running,
                        outputs: None,
                    }))
                    .chain(queue.pending.into_iter().map(|prompt| PromptListEntry {
                        prompt,
                        status: PromptStatus::Pending,
                        outputs: None,
                    }))
                    .collect_vec(),
            );
        }
        entries.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
        Ok(entries)
    }
}

#[derive(Debug, Clone)]
pub struct PromptListEntry {
    pub prompt: Prompt,
    pub status: PromptStatus,
    pub outputs: Option<Outputs>,
}

#[derive(Debug, Clone)]
pub enum PromptStatus {
    Completed,
    Pending,
    Running,
    Cancelled,
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
                if let Some(outputs) = entry.outputs {
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

impl Display for PromptStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PromptStatus::*;
        match self {
            Completed => write!(f, "{}", "completed".green()),
            Pending => write!(f, "{}", "pending".yellow()),
            Running => write!(f, "{}", "running".blue()),
            Cancelled => write!(f, "{}", "cancelled".red()),
        }
    }
}
