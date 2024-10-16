use std::{fs::File, io::Write, path::PathBuf};

use super::Run;
use clap::Args;

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
        let writer: Box<dyn Write> = if let Some(path) = self.output {
            Box::new(File::create(path)?)
        } else {
            Box::new(std::io::stdout())
        };

        let response: serde_json::Value = client.get(self.route).await?;

        if self.pretty {
            Ok(serde_json::to_writer_pretty(writer, &response)?)
        } else {
            Ok(serde_json::to_writer(writer, &response)?)
        }
    }
}
