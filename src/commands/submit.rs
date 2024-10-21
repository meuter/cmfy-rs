use super::Run;
use crate::io::{Input, JsonRead};
use clap::Args;
use cmfy::{dto, Client, Result};
use cmfy_nodes::KSampler;
use colored::Colorize;

/// Submits a batch of prompts to the server.
///
/// Reads a batch of prompts from a JSON file and submits it
/// to the server.
#[derive(Debug, Args)]
pub struct Submit {
    /// Input file containing the prompts in json format
    #[clap(default_value = "-")]
    input: Input,

    /// Reseeds the prompts before submission
    /// (assumes a KSampler node)
    #[clap(long, short, action, default_value_t = false)]
    reseed: bool,

    /// Allows to specify the number of times each prompt
    /// will be submitted.
    #[clap(long, short = 'n', action, default_value_t = 1)]
    count: usize,
}

impl Run for Submit {
    async fn run(mut self, client: Client) -> Result<()> {
        let prompts: Vec<dto::PromptNodes> = self.input.read_json()?;
        for mut prompt in prompts {
            if self.reseed {
                prompt.set_seed(rand::random())?;
            }
            for _ in 0..self.count {
                let response = client.submit(&prompt).await?;
                let index = format!("[{}]", response.number.to_string().bright_blue());
                println!("{:<15}{}", index, response.prompt_id);
            }
        }
        Ok(())
    }
}
