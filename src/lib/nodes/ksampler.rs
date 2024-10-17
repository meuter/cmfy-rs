use crate::{
    dto::{ClassType, PromptNodes},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KSamplerNode {
    pub cfg: f32,
    pub denoise: f32,
    pub sampler_name: String,
    pub scheduler: String,
    pub steps: u8,
    pub seed: u64,
    #[serde(flatten)]
    pub other: serde_json::Value,
}

impl ClassType for KSamplerNode {
    const CLASS_TYPE: &str = "KSampler";
}

pub trait KSampler {
    fn reseed(&mut self) -> Result<()>;
}

impl KSampler for PromptNodes {
    fn reseed(&mut self) -> Result<()> {
        self.change_first_by_class(|sampler: &mut KSamplerNode| {
            sampler.seed = rand::random();
        })
    }
}
