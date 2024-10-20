use cmfy_macros::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "LoraLoader")]
pub struct LoraLoaderInputs {
    pub lora_name: String,
    pub strength_clip: f32,
    pub strength_model: f32,
}


#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "KSampler")]
pub struct KSamplerInputs {
    steps: u8,
    seed: u64
}


#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "EmptyLatentImage")]
pub struct EmptyLatentImageInputs {
    pub batch_size: u8,
    pub height: usize,
    pub width: usize,
}

