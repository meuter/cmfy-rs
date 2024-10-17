use crate::io::JsonWrite;

use super::{list::PromptList, Run};
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

        let mut list = PromptList::default();
        if self.queue {
            let queue = client.queue().await?;
            list.append(&mut PromptList::from(queue));
        }
        if self.history {
            let history = client.history().await?;
            list.append(&mut PromptList::from(history));
        }

        // NOTE: The history and queue return submitted prompts with UUID, index, and possibly
        //       output nodes. The goal of capturing the prompts is to submit them for which
        //       we do not need the submit information. We just need the prompt nodes.
        let prompts = list.into_prompts().map(|prompt| prompt.nodes).collect_vec();
        self.output.write_json(&prompts, self.pretty)?;
        Ok(())
    }
}
