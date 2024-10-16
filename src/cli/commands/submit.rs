use super::Run;
use crate::io::Input;
use clap::Args;
use cmfy::dto;
use colored::Colorize;
use std::path::PathBuf;

/// Submits a batch of prompts to the server.
///
/// Reads a batch of prompts from a JSON file and submits it
/// to the server.
#[derive(Debug, Args)]
pub struct Submit {
    /// Input file containing the prompts in json format
    /// (if omitted, reads from standard input)
    input: Option<PathBuf>,
}

impl Run for Submit {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let input = Input::try_from(self.input)?;
        let prompts: Vec<dto::PromptNodes> = input.read_json()?;
        for prompt in &prompts {
            let response = client.submit(&prompt).await?;
            let index = format!("[{}]", response.number.to_string().bright_blue());
            println!("{:<15}{}", index, response.prompt_id);
        }
        Ok(())
    }
}
