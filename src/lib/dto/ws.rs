use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;

use super::Image;

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Status(Contents<Status>),
    Progress(Contents<Progress>),
    Executing(Contents<Executing>),
    Executed(Contents<Executed>),
    ExecutionStart(Contents<ExecutionStepData>),
    ExecutionSuccess(Contents<ExecutionStepData>),
    ExecutionCached(Contents<ExecutionStepData>),
    ExecutionInterrupted(Contents<ExecutionStepData>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct Contents<D> {
    pub data: D,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Status {
    pub status: StatusInner,
}

#[derive(Deserialize, Clone, Debug)]
pub struct StatusInner {
    pub exec_info: ExecInfo,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExecInfo {
    pub queue_remaining: usize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Progress {
    pub value: usize,
    pub max: usize,
    pub prompt_id: String,
    pub node: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Executing {
    pub node: Option<String>,
    pub display_node: Option<String>,
    pub prompt_id: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Executed {
    pub node: String,
    pub display_node: String,
    pub prompt_id: String,
    pub output: Outputs,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Outputs {
    pub images: Vec<Image>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExecutionStepData {
    pub prompt_id: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub nodes: Vec<String>,
}
