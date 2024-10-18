use super::{List, Run};
use crate::io::JsonWrite;
use clap::Args;
use clio::Output;
use itertools::Itertools;

/// Capture running and pending prompt to file.
///
/// Retrieves the running and pending prompts from
/// the server and saves them as json. These prompts
/// can then be re-queues using the 'submit' command.#[derive(Debug, Args)]
#[derive(Debug, Args)]
pub struct Capture {
    /// Capture all prompts from queue (running and pending)
    #[clap(long, short, action, default_value_t = false)]
    queue: bool,

    /// Capture all prompts from history (completed and cancelled)
    #[clap(long, short = 's', action, default_value_t = false)]
    history: bool,

    /// Capture all promts from both queue and history
    #[clap(long, short, action, default_value_t = true)]
    all: bool,

    /// Output path to store the captured prompt(s).
    #[clap(long, short, default_value = "-")]
    output: Output,

    /// Pretty prints the JSON output
    #[clap(long, action, default_value_t = false)]
    pretty: bool,
}

impl Run for Capture {
    async fn run(mut self, client: cmfy::Client) -> cmfy::Result<()> {
        if self.queue || self.history {
            self.all = false;
        }
        if self.all {
            self.queue = true;
            self.history = true;
        }

        // NOTE: The history and queue return submitted prompts with UUID, index, and possibly
        //       output nodes. The goal of capturing the prompts is to submit them for which
        //       we do not need the submit information. We just need the prompt nodes.
        let entries = List::collect_entries(&client, self.history, self.queue).await?;
        let prompts = entries.into_iter().map(|entries| entries.inner.nodes).collect_vec();
        self.output.write_json(&prompts, self.pretty)?;
        Ok(())
    }
}
