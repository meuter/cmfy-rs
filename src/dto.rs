use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub system: System,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub os: String,
    pub comfyui_version: String,
    pub python_version: String,
    pub pytorch_version: String,
    pub embedded_python: bool,
    pub argv: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub index: u32,
    pub vram_total: u64,
    pub vram_free: u64,
    pub torch_vram_total: u64,
    pub torch_vram_free: u64,
}
