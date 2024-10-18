use super::Run;
use clap::Args;

use cmfy::{dto::Outputs, History, Prompt, Queue};
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
}

#[derive(Debug, Default, Clone)]
pub struct PromptList(Vec<PromptListEntry>);

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
    async fn run(mut self, client: cmfy::Client) -> cmfy::Result<()> {
        use PromptStatus::*;

        if self.history || self.queue {
            self.all = false;
        }
        if self.all {
            self.history = true;
            self.queue = true;
        }

        let mut entries = vec![];
        if self.history {
            let history = client.history().await?;
            entries.extend(history.into_iter().map(|entry| {
                let status = if entry.cancelled() { Cancelled } else { Completed };
                let prompt = entry.prompt;
                let outputs = Some(entry.outputs);
                PromptListEntry { prompt, status, outputs }
            }))
        }
        if self.queue {
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

        for entry in entries {
            let prompt = entry.prompt;
            let index = format!("[{}]", prompt.index.to_string().bright_blue());
            print!("{:<15}{} ({})", index, prompt.uuid, entry.status);
            if self.images {
                if let Some(outputs) = entry.outputs {
                    for image in outputs.images() {
                        let url = client.url_for_image(image)?.to_string();
                        print!(" -> {}", url.cyan().underline());
                        break;
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

impl PromptList {
    pub fn append(&mut self, other: &mut PromptList) {
        self.0.append(&mut other.0);
    }

    pub fn into_prompts(self) -> impl Iterator<Item = Prompt> {
        self.0.into_iter().map(|entry| entry.prompt)
    }
}

impl IntoIterator for PromptList {
    type Item = PromptListEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.0.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
        self.0.into_iter()
    }
}

impl From<History> for PromptList {
    fn from(history: History) -> Self {
        use PromptStatus::*;
        let entries = history
            .into_iter()
            .map(|entry| {
                let status = if entry.cancelled() { Cancelled } else { Completed };
                let prompt = entry.prompt;
                let outputs = Some(entry.outputs);
                PromptListEntry { prompt, status, outputs }
            })
            .collect_vec();
        Self(entries)
    }
}

impl From<Queue> for PromptList {
    fn from(queue: Queue) -> Self {
        let entries = empty()
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
            .collect_vec();
        Self(entries)
    }
}

impl From<(History, Queue)> for PromptList {
    fn from((history, queue): (History, Queue)) -> Self {
        let mut entries = vec![];
        entries.append(&mut PromptList::from(history).0);
        entries.append(&mut PromptList::from(queue).0);
        Self(entries)
    }
}
