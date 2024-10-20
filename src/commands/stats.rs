use super::Run;
use clap::Args;
use cmfy::{Client, Result};
use colored::Colorize;
use humansize::{make_format, BINARY};

/// Displays basic statistics about client and server.
#[derive(Debug, Args)]
pub struct Stats;

impl Run for Stats {
    async fn run(self, client: Client) -> Result<()> {
        println!("{}", "client".yellow());
        println!("    name        : {}", env!("CARGO_PKG_NAME"));
        println!("    version     : {}", env!("CARGO_PKG_VERSION"));
        println!("    client_id   : {}", client.id);

        let stats = client.system_stats().await?;
        println!("{}", "server".yellow());
        println!("    url         : {}", client.base_url()?);
        println!("    os          : {}", stats.system.os);
        println!("    versions");
        println!(
            "        python  : {}{}",
            stats
                .system
                .python_version
                .split_whitespace()
                .next()
                .expect("malfored python version"),
            if stats.system.embedded_python { " (embedded)" } else { "" }
        );
        println!("        comfyui : {}", stats.system.comfyui_version);
        println!("        pytorch : {}", stats.system.pytorch_version);
        println!("    devices");
        let format_size = make_format(BINARY);
        for (index, device) in stats.devices.iter().enumerate() {
            println!(
                "        {:<8}: {} ({}/{})",
                format!("[{}]", index),
                device.name,
                format_size(device.vram_free),
                format_size(device.vram_total),
            );
        }
        Ok(())
    }
}
