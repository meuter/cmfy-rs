use clap::Args;
use cmfy::{Client, Error, Result};
use humantime::Duration;
use tokio::time::timeout;

use super::Run;

/// clear currently running prompt
#[derive(Debug, Args)]
pub struct Clear {
    /// Waits for the clear operation to be finished before returning.
    #[clap(long, short, action, default_value_t = true)]
    wait: bool,

    /// Maximum timeout (in seconds) to wait before bailing out.
    /// (only used with --wait)
    #[clap(long, short, action, default_value = "10s")]
    timeout: Duration,

    /// Amount of time between retries when waiting for the clear
    /// operation to complete.
    /// (only used with --wait)
    #[clap(long, short, action, default_value = "100ms")]
    retry: Duration,
}

impl Run for Clear {
    async fn run(self, client: Client) -> Result<()> {
        client.clear_queue().await?;
        client.cancel_running_prompt().await?;

        if self.wait {
            timeout(self.timeout.into(), async {
                loop {
                    let queue = client.queue().await?;
                    if queue.running.is_empty() && queue.pending.is_empty() {
                        break Ok::<(), Error>(());
                    }
                    tokio::time::sleep(self.retry.into()).await;
                }
            })
            .await??;
        }

        client.clear_history().await?;
        Ok(())
    }
}
