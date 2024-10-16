use super::Prompt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Queue {
    #[serde(rename = "queue_running")]
    pub running: Vec<Prompt>,
    #[serde(rename = "queue_pending")]
    pub pending: Vec<Prompt>,
}
