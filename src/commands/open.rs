use super::Run;
use clap::Args;
use cmfy::{Client, Result};

/// Open ComfyUI in a web browser.
#[derive(Debug, Args)]
pub struct Open;

impl Run for Open {
    async fn run(self, client: Client) -> Result<()> {
        let url = client.base_url()?;
        open::that(url.as_str())?;
        Ok(())
    }
}
