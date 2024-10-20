use super::Run;
use crate::io::{Input, JsonWrite, Output};
use clap::Args;
use cmfy::{dto::PromptNodes, Client, Result};

/// Extracts prompt information from a PNG generated
/// with Comfy UI, and outputs it as JSON.
#[derive(Debug, Args)]
pub struct Extract {
    /// Input file containing the prompts in json format
    #[clap(default_value = "-")]
    input: Input,

    /// Output path to store the captured prompt(s).
    #[clap(long, short, default_value = "-")]
    output: Output,

    /// Pretty prints the JSON output
    #[clap(long, action, default_value_t = false)]
    pretty: bool,
}

impl Run for Extract {
    async fn run(mut self, _: Client) -> Result<()> {
        let decoder = png::Decoder::new(self.input);
        let reader = decoder.read_info()?;
        let json = reader
            .info()
            .uncompressed_latin1_text
            .iter()
            .filter_map(|chunk| if chunk.keyword == "prompt" { Some(&chunk.text) } else { None })
            .next()
            .ok_or("could not find prompt in PNG".to_string())?;
        // self.output.write_json(value, pretty)
        let prompt = serde_json::from_str::<PromptNodes>(json)?;
        let prompts = [prompt];

        self.output.write_json(&prompts, self.pretty)?;
        Ok(())
    }
}
