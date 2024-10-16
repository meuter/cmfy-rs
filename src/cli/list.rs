use cmfy::{History, Prompt, Queue};
use colored::Colorize;
use itertools::Itertools;
use std::{fmt::Display, iter::empty};

enum Status {
    Completed,
    Pending,
    Running,
    Cancelled,
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

struct Entry {
    prompt: Prompt,
    status: Status,
}

pub struct PromptList(Vec<Entry>);

impl PromptList {
    pub fn display(mut self) {
        self.0.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
        for entry in self.0 {
            let prompt = entry.prompt;
            let index = format!("[{}]", prompt.index.to_string().bright_blue());
            println!("{:<15}{} ({})", index, prompt.uuid, entry.status);
        }
    }
}

impl From<History> for PromptList {
    fn from(history: History) -> Self {
        use Status::*;
        let entries = history
            .into_values()
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
