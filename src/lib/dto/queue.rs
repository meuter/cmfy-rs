use super::{Prompt, PromptBatch, PromptBatchEntry};
use crate::MarkAs;
use itertools::Itertools;
use serde::Deserialize;
use std::iter::empty;

#[derive(Debug, Clone, Deserialize)]
pub struct Queue {
    #[serde(rename = "queue_running")]
    pub running: Vec<Prompt>,
    #[serde(rename = "queue_pending")]
    pub pending: Vec<Prompt>,
}

impl Queue {
    pub fn into_batch_entries(self) -> impl Iterator<Item = PromptBatchEntry> {
        use crate::Status::*;
        empty()
            .chain(self.running.into_iter().map(|prompt| prompt.mark_as(Running)))
            .chain(self.pending.into_iter().map(|prompt| prompt.mark_as(Pending)))
    }
}

impl From<Queue> for PromptBatch {
    fn from(queue: Queue) -> Self {
        let mut result = queue.into_batch_entries().collect_vec();
        result.sort_by(|l, r| l.inner.index.cmp(&r.inner.index));
        result
    }
}
