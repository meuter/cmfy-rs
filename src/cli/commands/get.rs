use std::path::PathBuf;
use super::Run;
use clap::Args;
use crate::io::Output;

/// Display GET request raw json output.
///
/// Performs a get request to the server, and displays
/// the raw json output. For more information on which
/// routes are available, refer to:
///
/// https://docs.comfy.org/essentials/comms_routes
#[derive(Debug, Args)]
pub struct Get {
    /// the route, e.g. "/history"
    route: String,

    /// Output path to store the captured prompt(s).
    /// (if omitted, writes to standard output)
    #[clap(long, short, verbatim_doc_comment)]
    output: Option<PathBuf>,

    /// Pretty prints the JSON output
    #[clap(long, short, action, default_value_t = false)]
    pretty: bool,
}

impl Run for Get {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let response: serde_json::Value = client.get(self.route).await?;
        let output = Output::try_from(self.output)?;
        output.write_json(&response, self.pretty)
    }
}
