use super::Run;
use clap::Args;
use cmfy::{Client, Result};
use humansize::{make_format, BINARY};

/// Displays basic statistics about the server.
#[derive(Debug, Args)]
pub struct Stats;

impl Run for Stats {
    async fn run(self, client: Client) -> Result<()> {
        let stats = client.system_stats().await?;
        println!("versions:");
        println!(
            "    python  : {}",
            stats
                .system
                .python_version
                .split_whitespace()
                .next()
                .expect("malfored python version")
        );
        println!("    comfyui : {}", stats.system.comfyui_version);
        println!("    pytorch : {}", stats.system.pytorch_version);
        println!("devices:");
        let format_size = make_format(BINARY);
        for device in &stats.devices {
            println!(
                "    {} ({}/{})",
                device.name,
                format_size(device.vram_free),
                format_size(device.vram_total),
            );
        }
        Ok(())
    }
}
