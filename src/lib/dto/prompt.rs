use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub index: u64,
    pub uuid: String,
    pub nodes: PromptNodes,
    pub png_info: serde_json::Value,
    pub output_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PromptNodes(pub BTreeMap<String, Node<serde_json::Value>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<I> {
    pub class_type: String,
    pub inputs: I,
}
