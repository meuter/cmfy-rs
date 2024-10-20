use cmfy_macros::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "LoraLoader")]
pub struct LoraLoaderInputs {
    pub lora_name: String,
    pub strength_clip: f32,
    pub strength_model: f32,
}

