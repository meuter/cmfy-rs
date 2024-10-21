use super::Run;
use clap::Args;
use cmfy::{
    dto::{websocket::Message, Outputs},
    Client, Prompt, Result, Status,
};
use cmfy_nodes::KSampler;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone, Args)]
pub struct Monitor;

pub struct PromptProgressBars {
    pub multi: MultiProgress,
    pub by_id: HashMap<String, ProgressBar>,
    pub client: Client,
}

impl PromptProgressBars {
    pub fn from_client(client: Client) -> Self {
        let multi = MultiProgress::new();
        let by_id = HashMap::new();
        Self { multi, by_id, client }
    }

    pub fn set_status(&mut self, prompt_id: impl AsRef<str>, status: Status<Outputs>) -> Result<()> {
        const TEMPLATE_WITH_STEPS: &str = "{prefix} {msg} {pos:>3}/{len:3} [{elapsed_precise}]";
        const TEMPLATE_WITHOUT_STEPS: &str = "{prefix} {msg}";

        let bar = self.by_id.get_mut(prompt_id.as_ref()).expect("could not find prompt?");
        bar.set_message(format!("{:<20}", format!("({})", status)));
        match &status {
            Status::Completed(outputs) => {
                if let Some(image) = outputs.images().next() {
                    let url = self.client.url_for_image(image)?.to_string();
                    let status = format!("({})", status);
                    bar.set_message(format!("{:<20} -> {}", status, url));
                }
                bar.set_style(ProgressStyle::with_template(TEMPLATE_WITHOUT_STEPS)?);
                bar.disable_steady_tick();
                bar.finish();
            }
            Status::Pending => {
                bar.set_style(ProgressStyle::with_template(TEMPLATE_WITHOUT_STEPS)?);
                bar.disable_steady_tick();
                bar.set_message(format!("{:<20}", format!("({})", status)));
            }
            Status::Running => {
                bar.set_style(ProgressStyle::with_template(TEMPLATE_WITH_STEPS)?);
                bar.enable_steady_tick(Duration::from_millis(100));
                bar.set_message(format!("{:<20}", format!("({})", status)));
            }
            Status::Cancelled => {
                bar.disable_steady_tick();
                bar.set_style(ProgressStyle::with_template(TEMPLATE_WITHOUT_STEPS)?);
                bar.set_message(format!("{:<20}", format!("({})", status)));
            }
        }
        Ok(())
    }

    pub fn set_position(&mut self, prompt_id: impl AsRef<str>, position: usize) -> Result<()> {
        let bar = self.by_id.get_mut(prompt_id.as_ref()).expect("could not find prompt?");
        bar.set_position(position as u64);
        Ok(())
    }

    pub fn register_prompt(&mut self, prompt: &Prompt) -> Result<()> {
        let steps = prompt.nodes.steps()?;
        let bar = self.multi.add(ProgressBar::new(steps as u64));
        let index = format!("[{}]", prompt.index.to_string().bright_blue());
        bar.set_prefix(format!("{:<15}{}", index, prompt.uuid));
        self.by_id.insert(prompt.uuid.clone(), bar);
        Ok(())
    }
}

impl Run for Monitor {
    async fn run(self, client: Client) -> Result<()> {
        let mut bars = PromptProgressBars::from_client(client.clone());

        let entries = client.collect_prompt_batch(true, true).await?;
        for entry in entries {
            let prompt = entry.inner;
            let status = entry.status;
            bars.register_prompt(&prompt)?;
            bars.set_status(&prompt.uuid, status)?;
        }

        let mut message_stream = client.listen().await?;
        while let Some(message) = message_stream.next_json::<Message>().await? {
            match message {
                Message::Status(_contents) => {}
                Message::Progress(contents) => {
                    bars.set_status(&contents.data.prompt_id, Status::Running)?;
                    bars.set_position(&contents.data.prompt_id, contents.data.value)?;
                }
                Message::Executing(_contents) => {}
                Message::Executed(_contents) => {}
                Message::ExecutionStart(contents) => {
                    bars.set_position(&contents.data.prompt_id, 0)?;
                    bars.set_status(&contents.data.prompt_id, Status::Running)?;
                }
                Message::ExecutionSuccess(contents) => {
                    //
                    for entry in client.collect_prompt_batch(true, false).await? {
                        if entry.inner.uuid == contents.data.prompt_id {
                            bars.set_status(&contents.data.prompt_id, entry.status)?;
                        }
                    }
                }
                Message::ExecutionCached(_contents) => {}
                Message::ExecutionInterrupted(contents) => {
                    //
                    bars.set_status(&contents.data.prompt_id, Status::Cancelled)?;
                }
            }
        }
        Ok(())
    }
}
