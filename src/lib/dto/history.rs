use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;
use std::collections::BTreeMap;

pub type History = BTreeMap<String, HistoryLogEntry>;

#[derive(Debug, Clone, Deserialize)]
pub struct HistoryLogEntry {
    pub prompt: Prompt,
    pub outputs: Outputs,
    pub status: Status,
    pub meta: Meta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Prompt {
    pub index: u64,
    pub uuid: String,
    pub nodes: serde_json::Value,
    pub png_info: serde_json::Value,
    pub output_nodes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct Outputs(pub BTreeMap<String, serde_json::Value>);

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
