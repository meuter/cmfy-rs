use clap::Args;
use itertools::Itertools;
use std::{fs::File, io::Write, path::PathBuf};

use super::{list::PromptList, Run};

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
    #[clap(long, short='s', action, default_value_t = false)]
    history: bool,

    /// Capture all promts from both queue and history
    #[clap(long, short, action, default_value_t = true)]
    all: bool,

    /// Output path to store the captured prompt(s).
    /// (if omitted, writes to standard output)
    #[clap(long, short, verbatim_doc_comment)]
    output: Option<PathBuf>,

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

        let writer: Box<dyn Write> = if let Some(path) = self.output {
            Box::new(File::create(path)?)
        } else {
            Box::new(std::io::stdout())
        };
        let prompts = list.into_prompts().collect_vec();
        if self.pretty {
            Ok(serde_json::to_writer_pretty(writer, &prompts)?)
        } else {
            Ok(serde_json::to_writer(writer, &prompts)?)
        }
    }
}
