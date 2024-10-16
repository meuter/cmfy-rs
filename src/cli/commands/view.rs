use clap::Args;
use cmfy::{Client, Result};
use itertools::Itertools;

use super::Run;

/// Open images from completed prompts in a browser
#[derive(Debug, Args)]
pub struct View {
    /// Select a range of prompt indices,
    /// e.g. '1,2,3' or '4-5' or '1,3,4-6'
    #[clap(action, default_value = None)]
    range: Option<String>,
}

impl Run for View {
    async fn run(self, client: Client) -> Result<()> {
        let history = client.history().await?;
        let mut set = tokio::task::JoinSet::new();
        let entries = if let Some(range) = self.range {
            let range: Vec<u64> = range_parser::parse(&range)?;
            history
                .into_iter()
                .filter(|entry| range.contains(&entry.prompt.index))
                .collect_vec()
        } else {
            history.into_iter().collect_vec()
        };
        for entry in entries {
            for image in entry.into_output_images() {
                let url = client.url_for_image(&image)?;
                set.spawn(async move {
                    println!("{}", url);
                    open::that(url.to_string())
                });
            }
        }

        set.join_all().await.into_iter().collect::<std::io::Result<Vec<_>>>()?;
        Ok(())
    }
}
