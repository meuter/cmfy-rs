use crate::{MarkAs, WithStatus};

use super::Prompt;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{btree_map::IntoValues, BTreeMap};

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct History(pub BTreeMap<String, HistoryLogEntry>);

#[derive(Debug, Clone, Deserialize)]
pub struct HistoryLogEntry {
    pub prompt: Prompt,
    pub outputs: Outputs,
    pub status: Status,
    pub meta: Meta,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Outputs(pub BTreeMap<String, Output>);

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Output {
    Images { images: Vec<Image> },
    Other(BTreeMap<String, Vec<serde_json::Value>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub filename: String,
    pub subfolder: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    pub status_str: String,
    pub completed: bool,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub kind: MessageKind,
    pub data: MessageData,
}

#[derive(PartialEq, Eq, Debug, Clone, Deserialize)]
pub enum MessageKind {
    #[serde(rename = "execution_start")]
    Start,
    #[serde(rename = "execution_cached")]
    Cached,
    #[serde(rename = "execution_success")]
    Success,
    #[serde(rename = "execution_interrupted")]
    Interruped,
    #[serde(rename = "execution_error")]
    Error,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageData {
    pub prompt_id: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Meta(pub BTreeMap<u32, Metadata>);

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub node_id: String,
    pub display_node: String,
    pub parent_node: Option<String>,
    pub real_node_id: String,
}

impl MarkAs for Prompt {}
pub type PromptBatchEntry = WithStatus<Prompt, Outputs>;
pub type PromptBatch = Vec<PromptBatchEntry>;

impl History {
    pub fn into_batch_entries(self) -> impl Iterator<Item = PromptBatchEntry> {
        self.into_iter().map(|entry| entry.into())
    }
}

impl IntoIterator for History {
    type Item = HistoryLogEntry;
    type IntoIter = IntoValues<String, HistoryLogEntry>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

impl From<HistoryLogEntry> for PromptBatchEntry {
    fn from(entry: HistoryLogEntry) -> Self {
        if entry.status.messages.iter().any(|msg| msg.kind == MessageKind::Interruped) {
            entry.prompt.mark_as(crate::Status::Cancelled)
        } else {
            entry.prompt.mark_as(crate::Status::Completed(entry.outputs))
        }
    }
}

impl From<History> for PromptBatch {
    fn from(history: History) -> Self {
        let mut result = history.into_batch_entries().collect_vec();
        result.sort_by(|l, r| l.inner.index.cmp(&r.inner.index));
        result
    }
}

impl Outputs {
    pub fn images(&self) -> impl Iterator<Item = &Image> {
        self.0
            .values()
            .filter_map(|output| {
                if let Output::Images { images } = output {
                    Some(images)
                } else {
                    None
                }
            })
            .flatten()
    }
}
