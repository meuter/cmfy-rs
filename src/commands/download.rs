use super::Run;
use clap::Args;
use cmfy::{Client, Result};
use itertools::Itertools;
use std::io::Cursor;

/// Download images from completed prompts locally
#[derive(Debug, Args)]
pub struct Download {
    /// Select a range of prompt indices,
    /// e.g. '1,2,3' or '4-5' or '1,3,4-6'
    #[clap(action, default_value = None)]
    range: Option<String>,

    /// Remove prompts from history after
    #[clap(short, long, action, default_value_t = false)]
    clear: bool,
}

impl Run for Download {
    async fn run(self, client: Client) -> Result<()> {
        let history = client.history().await?;
        let entries = if let Some(range) = self.range {
            let range: Vec<u64> = range_parser::parse(&range)?;
            history
                .into_iter()
                .filter(|entry| range.contains(&entry.prompt.index))
                .collect_vec()
        } else {
            history.into_iter().collect_vec()
        };

        let mut set = tokio::task::JoinSet::new();
        for entry in &entries {
            for image in entry.outputs.images() {
                let url = client.url_for_image(image)?;
                let filename = image.filename.clone();
                set.spawn(async move {
                    let response = reqwest::get(url.clone()).await?;
                    let mut file = std::fs::File::create(&filename)?;
                    let mut content = Cursor::new(response.bytes().await?);
                    std::io::copy(&mut content, &mut file)?;
                    println!("{} -> {}", url, filename);
                    Ok(())
                });
            }
        }
        set.join_all().await.into_iter().collect::<Result<Vec<_>>>()?;

        if self.clear {
            for entry in &entries {
                client.delete_from_history(&entry.prompt.uuid).await?;
            }
        }
        Ok(())
    }
}
