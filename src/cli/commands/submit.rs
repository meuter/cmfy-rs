use super::Run;
use crate::io::{Input, JsonRead};
use clap::Args;
use cmfy::dto;
use colored::Colorize;

// TODO: reseed
// TODO: submit each multiple times

/// Submits a batch of prompts to the server.
///
/// Reads a batch of prompts from a JSON file and submits it
/// to the server.
#[derive(Debug, Args)]
pub struct Submit {
    /// Input file containing the prompts in json format
    #[clap(default_value = "-")]
    input: Input,
}

impl Run for Submit {
    async fn run(mut self, client: cmfy::Client) -> cmfy::Result<()> {
        let prompts: Vec<dto::PromptNodes> = self.input.read_json()?;
        for prompt in &prompts {
            let response = client.submit(prompt).await?;
            let index = format!("[{}]", response.number.to_string().bright_blue());
            println!("{:<15}{}", index, response.prompt_id);
        }
        Ok(())
    }
}
