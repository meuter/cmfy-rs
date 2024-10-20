use crate::{
    dto::{ClassType, PromptNodes},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoraLoaderNode {
    pub lora_name: String,
    pub strength_clip: f32,
    pub strength_model: f32,
}

impl ClassType for LoraLoaderNode {
    const CLASS_TYPE: &str = "LoraLoader";
}

pub trait LoraLoader {
    fn lora_name(&self) -> Result<String>;
    fn set_lora_name(&mut self, lora_name: impl AsRef<str>) -> Result<()>;
}

impl LoraLoader for PromptNodes {
    fn lora_name(&self) -> Result<String> {
        let (_, loader) = self.first_by_class::<LoraLoaderNode>()?;
        Ok(loader.lora_name)
    }

    fn set_lora_name(&mut self, lora_name: impl AsRef<str>) -> Result<()> {
        self.change_first_by_class(|loader: &mut LoraLoaderNode| loader.lora_name = lora_name.as_ref().to_string())
    }
}
