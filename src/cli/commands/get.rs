use super::Run;
use clap::Args;

/// Display GET request raw json output.
#[derive(Debug, Args)]
pub struct Get {
    /// the route, e.g. "/history"
    route: String,
}

impl Run for Get {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let response: serde_json::Value = client.get(self.route).await?;
        serde_json::to_writer(std::io::stdout(), &response)?;
        Ok(())
    }
}
