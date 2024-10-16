use cmfy::{History, Prompt, Queue};
use itertools::Itertools;
use std::iter::empty;

pub struct PromptListEntry {
    prompt: Prompt,
    status: &'static str,
}

pub struct PromptList(Vec<PromptListEntry>);

impl PromptList {
    pub fn display(mut self) {
        self.0.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
        for entry in self.0 {
            let prompt = entry.prompt;
            let index = format!("[{}]", prompt.index);
            println!("{:<5}{} ({})", index, prompt.uuid, entry.status);
        }
    }
}

impl From<History> for PromptList {
    fn from(history: History) -> Self {
        let entries = history
            .into_values()
            .map(|entry| {
                let status = if entry.cancelled() { "cancelled" } else { "completed" };
                let prompt = entry.prompt;
                PromptListEntry { prompt, status }
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
                status: "running",
            }))
            .chain(queue.pending.into_iter().map(|prompt| PromptListEntry {
                prompt,
                status: "pending",
            }))
            .collect_vec();
        Self(entries)
    }
}
