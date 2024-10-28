use super::Run;
use clap::Args;
use cmfy::{
    dto::{websocket as ws, websocket::Message},
    Client, Prompt, Result, Status,
};
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::{collections::HashMap, time::Duration};

/// Monitors the progress on ongoing prompts.
#[derive(Debug, Clone, Args)]
pub struct Monitor;

impl Run for Monitor {
    async fn run(self, client: Client) -> Result<()> {
        let mut stream = client.listen().await?;
        let mut bars = AllStatusProgressBars::default();
        let timeout = Duration::from_secs(1);

        loop {
            match stream.next_json_with_timeout(timeout).await {
                Ok(Ok(Some(message))) => bars.dispatch_message(&client, message).await?,
                Ok(Ok(None)) => return Ok(()),
                Ok(Err(error)) => return Err(error),
                Err(_timeout) => bars.refresh(&client).await?,
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// AllStatusProgressBars
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone)]
pub struct AllStatusProgressBars {
    pub multi: MultiProgress,
    pub by_id: HashMap<String, ProgressBar>,
}

struct AllStyles;

impl AllStyles {
    pub fn with_message() -> ProgressStyle {
        let template = "{prefix} {msg}";
        ProgressStyle::with_template(template).unwrap()
    }
    pub fn with_message_and_timing() -> ProgressStyle {
        let template = "{prefix} {msg} -> [{elapsed_precise} < {duration_precise}]";
        ProgressStyle::with_template(template).unwrap()
    }
    pub fn with_message_steps_and_timing() -> ProgressStyle {
        let template = "{prefix} {msg} -> {pos:>2}/{len:2} [{elapsed_precise} < {duration_precise}]";
        ProgressStyle::with_template(template).unwrap()
    }
}

impl AllStatusProgressBars {
    pub async fn dispatch_message(&mut self, client: &Client, message: ws::Message) -> Result<()> {
        use Message::*;
        match message {
            Status(_) => self.refresh(client).await,
            Progress(contents) => {
                let bar = self.get_progress_bar(&contents.data.prompt_id).unwrap();
                bar.set_style(AllStyles::with_message_steps_and_timing());
                bar.set_length(contents.data.max as u64);
                bar.set_position(contents.data.value as u64);
                Ok(())
            }
            ExecutionStart(contents) => {
                let bar = self.get_progress_bar(&contents.data.prompt_id).unwrap();
                bar.reset_elapsed();
                bar.reset_eta();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn refresh(&mut self, client: &Client) -> Result<()> {
        let entries = client.collect_prompt_batch(true, true).await?;

        let to_remove = self
            .by_id
            .keys()
            .filter(|prompt_id| !entries.iter().any(|entry| entry.inner.uuid == **prompt_id))
            .map(String::clone)
            .collect_vec();

        for prompt_id in to_remove {
            let bar = self.by_id.remove(&prompt_id).unwrap();
            bar.finish_and_clear();
            self.multi.remove(&bar);
        }

        for entry in entries {
            let bar = self.get_progress_bar(&entry.inner.uuid).unwrap_or_else(|| {
                let prompt: &Prompt = &entry.inner;
                let bar = self.multi.add(ProgressBar::new(0));
                let index = format!("[{}]", prompt.index.to_string().bright_blue());
                bar.set_prefix(format!("{:<15}{}", index, prompt.uuid));
                self.by_id.insert(prompt.uuid.clone(), bar.clone());
                bar
            });

            let colored_status = format!("({})", entry.status.colored());
            match &entry.status {
                Status::Completed(outputs) => {
                    bar.set_style(AllStyles::with_message());
                    bar.disable_steady_tick();
                    if let Some(image) = outputs.images().next() {
                        let url = client.url_for_image(image);
                        bar.set_message(format!("{:<20} -> {}", colored_status, url.to_string().cyan().underline()));
                    } else {
                        bar.set_message(format!("{:<20}", colored_status));
                    }
                    bar.finish();
                }
                Status::Pending | Status::Cancelled => {
                    bar.set_style(AllStyles::with_message());
                    bar.disable_steady_tick();
                    bar.set_message(format!("{:<20}", colored_status));
                }
                Status::Running => {
                    if bar.length().is_some() {
                        bar.set_style(AllStyles::with_message_steps_and_timing());
                    } else {
                        bar.set_style(AllStyles::with_message_and_timing());
                    }
                    bar.enable_steady_tick(Duration::from_millis(100));
                    bar.set_message(format!("{:<20}", colored_status));
                }
            };
        }

        Ok(())
    }

    pub fn get_progress_bar(&self, prompt_id: impl AsRef<str>) -> Option<ProgressBar> {
        self.by_id.get(prompt_id.as_ref()).cloned()
    }
}
