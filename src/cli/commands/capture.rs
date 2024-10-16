use clap::Args;
use std::{fs::File, io::Write, path::PathBuf};

use super::Run;

/// Capture running and pending prompt to file.
///
/// Retrieves the running and pending prompts from
/// the server and saves them as json. These prompts
/// can then be re-queues using the 'submit' command.#[derive(Debug, Args)]
#[derive(Debug, Args)]
pub struct Capture {
    /// Capture completed prompts
    #[clap(long, short, action, default_value_t = false)]
    completed: bool,

    /// Capture pending prompts.
    #[clap(long, short, action, default_value_t = false)]
    pending: bool,

    /// Capture currently running prompts.
    #[clap(long, short, action, default_value_t = false)]
    running: bool,

    /// Capture both running and pending prompts.
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
        if self.running || self.pending || self.completed {
            self.all = false;
        }
        if self.all {
            self.completed = true;
            self.pending = true;
            self.running = true;
        }

        let mut prompts = Vec::new();
        if self.running || self.pending {
            let mut queue = client.queue().await?;
            if self.running {
                prompts.append(&mut queue.running);
            }
            if self.pending {
                prompts.append(&mut queue.pending);
            }
        }

        if self.completed {
            let mut history = client.history().await?.into_values().map(|entry| entry.prompt).collect();
            prompts.append(&mut history)
        }

        let writer: Box<dyn Write> = if let Some(path) = self.output {
            Box::new(File::create(path)?)
        } else {
            Box::new(std::io::stdout())
        };

        if self.pretty {
            Ok(serde_json::to_writer_pretty(writer, &prompts)?)
        } else {
            Ok(serde_json::to_writer(writer, &prompts)?)
        }
    }
}
