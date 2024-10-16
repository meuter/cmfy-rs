use super::Run;
use crate::io::Output;
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
    #[clap(long, short)]
    output: Output,

    /// Pretty prints the JSON output
    #[clap(long, short, action, default_value_t = false)]
    pretty: bool,
}

impl Run for Get {
    async fn run(self, client: cmfy::Client) -> cmfy::Result<()> {
        let response: serde_json::Value = client.get(self.route).await?;
        self.output.write_json(&response, self.pretty)
    }
}
