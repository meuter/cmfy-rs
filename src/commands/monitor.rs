use super::Run;
use clap::Args;
use cmfy::{
    dto::{websocket as ws, websocket::Message, Outputs},
    Client, Prompt, Result, Status,
};
use cmfy_nodes::KSampler;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone, Args)]
pub struct Monitor;

impl Run for Monitor {
    async fn run(self, client: Client) -> Result<()> {
        let mut stream = client.listen().await?;
        let mut bars = AllStatusProgressBars::default();
        let timeout = Duration::from_millis(500);

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

impl AllStatusProgressBars {
    pub async fn dispatch_message(&mut self, client: &Client, message: ws::Message) -> Result<()> {
        match message {
            Message::Status(_) => self.refresh(client).await,
            Message::Progress(contents) => {
                let bar = self.get_progress_bar(&contents.data.prompt_id).unwrap();
                bar.set_progress(contents.data.value)
            }
            _ => Ok(()),
        }
    }

    pub async fn refresh(&mut self, client: &Client) -> Result<()> {
        let entries = client.collect_prompt_batch(true, true).await?;

        for entry in &entries {
            // TODO: avoid duplication of set_status
            if let Some(bar) = self.get_progress_bar(&entry.inner.uuid) {
                bar.set_status(client, entry.status.clone())?;
            } else {
                let bar = self.create_progress_bar(&entry.inner)?;
                bar.set_status(client, entry.status.clone())?;
            }
        }

        // NOTE: the bar for prompts that are not in the batch anymore should be removed
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

        Ok(())
    }

    pub fn create_progress_bar(&mut self, prompt: &Prompt) -> Result<ProgressBar> {
        let steps = prompt.nodes.steps()?;
        let bar = self.multi.add(ProgressBar::new(steps as u64));
        let index = format!("[{}]", prompt.index.to_string().bright_blue());
        bar.set_prefix(format!("{:<15}{}", index, prompt.uuid));
        self.by_id.insert(prompt.uuid.clone(), bar.clone());
        Ok(bar)
    }

    pub fn get_progress_bar(&self, prompt_id: impl AsRef<str>) -> Option<ProgressBar> {
        self.by_id.get(prompt_id.as_ref()).cloned()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// StatusProgressBar
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

trait StatusProgressBar {
    // TODO: don't display the steps unless we have progress information to report
    const TEMPLATE_WITH_STEPS: &str = "{prefix} {msg} -> steps: {pos:>2}/{len:2} [{elapsed_precise} < {duration_precise}]";
    const TEMPLATE_WITHOUT_STEPS: &str = "{prefix} {msg}";
    const TEMPLATE_RUNNING_WO_STEPS: &str = "{prefix} {msg} -> [{elapsed_precise} < {duration_precise}]";
    const TEMPLATE_RUNNING_WITH_STEPS: &str = "{prefix} {msg} -> {pos:>2}/{len:2} [{elapsed_precise} < {duration_precise}]";

    fn set_progress(&self, position: usize) -> Result<()>;
    fn set_status(&self, client: &Client, status: Status<Outputs>) -> Result<()>;
}

impl StatusProgressBar for ProgressBar {
    fn set_progress(&self, position: usize) -> Result<()> {
        let style = ProgressStyle::with_template(Self::TEMPLATE_RUNNING_WITH_STEPS)?;
        self.set_style(style);
        self.set_position(position as u64);
        Ok(())
    }

    fn set_status(&self, client: &Client, status: Status<Outputs>) -> Result<()> {
        self.set_message(format!("{:<20}", format!("({})", status.colored())));
        match &status {
            Status::Completed(outputs) => {
                if let Some(image) = outputs.images().next() {
                    let url = client.url_for_image(image)?.to_string();
                    let status = format!("({})", status.colored());
                    self.set_message(format!("{:<20} -> {}", status, url.cyan().underline()));
                }
                self.set_style(ProgressStyle::with_template(Self::TEMPLATE_WITHOUT_STEPS)?);
                self.disable_steady_tick();
                self.finish();
            }
            Status::Pending | Status::Cancelled => {
                self.set_style(ProgressStyle::with_template(Self::TEMPLATE_WITHOUT_STEPS)?);
                self.disable_steady_tick();
                self.set_message(format!("{:<20}", format!("({})", status.colored())));
            }
            Status::Running => {
                self.set_style(ProgressStyle::with_template(Self::TEMPLATE_RUNNING_WO_STEPS)?);
                self.enable_steady_tick(Duration::from_millis(100));
                self.set_message(format!("{:<20}", format!("({})", status.colored())));
            }
        }
        Ok(())
    }
}
