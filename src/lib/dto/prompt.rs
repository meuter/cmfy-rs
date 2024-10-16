use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Prompt {
    pub index: u64,
    pub uuid: String,
    pub nodes: serde_json::Value,
    pub png_info: serde_json::Value,
    pub output_nodes: Vec<String>,
}
