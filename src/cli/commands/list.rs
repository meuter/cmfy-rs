use super::Run;
use clap::Args;

use cmfy::{History, Prompt, Queue};
use colored::Colorize;
use itertools::Itertools;
use std::{fmt::Display, iter::empty};

/// List all prompts from history and queue
#[derive(Debug, Args)]
pub struct List {}

#[derive(Debug, Default, Clone)]
pub struct PromptList(Vec<Entry>);

#[derive(Debug, Clone)]
struct Entry {
    prompt: Prompt,
    status: Status,
}

#[derive(Debug, Clone)]
enum Status {
    Completed,
    Pending,
    Running,
    Cancelled,
}

impl Run for List {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let history = client.history().await?;
        let queue = client.queue().await?;
        PromptList::from((history, queue)).display();
        Ok(())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Status::*;
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

    pub fn display(mut self) {
        self.0.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
        for entry in self.0 {
            let prompt = entry.prompt;
            let index = format!("[{}]", prompt.index.to_string().bright_blue());
            println!("{:<15}{} ({})", index, prompt.uuid, entry.status);
        }
    }

    pub fn into_prompts(self) -> impl Iterator<Item = Prompt> {
        self.0.into_iter().map(|entry| entry.prompt)
    }
}

impl From<History> for PromptList {
    fn from(history: History) -> Self {
        use Status::*;
        let entries = history
            .into_iter()
            .map(|entry| {
                let status = if entry.cancelled() { Cancelled } else { Completed };
                let prompt = entry.prompt;
                Entry { prompt, status }
            })
            .collect_vec();
        Self(entries)
    }
}

impl From<Queue> for PromptList {
    fn from(queue: Queue) -> Self {
        let entries = empty()
            .chain(queue.running.into_iter().map(|prompt| Entry {
                prompt,
                status: Status::Running,
            }))
            .chain(queue.pending.into_iter().map(|prompt| Entry {
                prompt,
                status: Status::Pending,
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
