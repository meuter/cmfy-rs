use cmfy_macros::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "LoraLoader")]
pub struct LoraLoaderInputs {
    pub lora_name: String,
    pub strength_clip: f32,
    pub strength_model: f32,
    #[serde(flatten)]
    #[node_input(skip)]
    pub other: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "KSampler")]
pub struct KSamplerInputs {
    pub cfg: f32,
    pub denoise: f32,
    pub sampler_name: String,
    pub scheduler: String,
    pub steps: u8,
    pub seed: u64,
    #[serde(flatten)]
    #[node_input(skip)]
    pub other: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "EmptyLatentImage")]
pub struct EmptyLatentImageInputs {
    pub batch_size: u8,
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Node)]
#[node(class_type = "CLIPTextEncode")]
pub struct ClipTextEncodeInput {
    pub text: String,
    pub clip: serde_json::Value,
}

