use super::Run;
use crate::io::{Input, JsonRead};
use clap::Args;
use cmfy::{dto, Client, Result};
use cmfy_nodes::{EmptyLatentImage, KSampler};
use colored::Colorize;
use itertools::Itertools;

/// Submits a batch of prompts to the server.
///
/// Reads a batch of prompts from a JSON file and submits it
/// to the server.
#[derive(Debug, Args)]
pub struct Submit {
    /// Input file containing the prompts in json format
    #[clap(default_value = "-")]
    input: Input,

    /// Reseeds the prompts at random before submission
    /// (assumes a KSampler node)
    #[clap(long, action, default_value_t = false)]
    reseed: bool,

    /// Set the size of the prompts before submission
    /// (assumes a EmptyLatentImage node)
    #[clap(long, action, value_name = "WIDTHxHEIGHT(xBATCH)")]
    size: Option<String>,

    /// Sets the number of steps of the prompts before submission
    /// (assumes a KSampler node)
    #[clap(long, action)]
    steps: Option<u8>,

    /// Allows to specify the number of times each prompt
    /// will be submitted.
    #[clap(long, short = 'n', action, default_value_t = 1)]
    count: usize,
}

impl Run for Submit {
    async fn run(mut self, client: Client) -> Result<()> {
        let prompts: Vec<dto::PromptNodes> = self.input.read_json()?;
        for mut prompt in prompts {
            if let Some(size) = &self.size {
                let split = size.split("x").collect_vec();
                if split.len() != 2 && split.len() != 3 {
                    Err(format!("size: could not parse '{}'", size))?;
                }
                if split.len() >= 2 {
                    let width = split[0].parse()?;
                    let height = split[1].parse()?;
                    prompt.set_width(width)?;
                    prompt.set_height(height)?;
                }
                if split.len() == 3 {
                    let batch = split[2].parse()?;
                    prompt.set_batch_size(batch)?;
                }
            }
            if let Some(steps) = self.steps {
                prompt.set_steps(steps)?;
            }
            for _ in 0..self.count {
                if self.reseed {
                    prompt.set_seed(rand::random())?;
                }
                let response = client.submit(&prompt).await?;
                let index = format!("[{}]", response.number.to_string().bright_blue());
                println!("{:<15}{}", index, response.prompt_id);
            }
        }
        Ok(())
    }
}
