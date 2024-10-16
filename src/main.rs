mod client;
mod dto;
mod error;

use clap::{Parser, Subcommand};
use client::Client;
use dto::SystemStats;
use humansize::{make_format, BINARY};
use std::error::Error;

#[derive(Parser, Debug)]
#[clap(version)]
#[command(infer_subcommands = true)]
struct Cli {
    /// ip of the server
    #[arg(short, long, env = "COMFY_SERVER", value_name = "SERVER", default_value = "localhost")]
    server: String,

    /// port of the server
    #[arg(short, long, env = "COMFY_PORT", value_name = "PORT", default_value_t = 8188)]
    port: u32,

    /// command to execute
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Displays basic statistics about the server.
    Stats,

    /// Lists prompts from history
    History,

    /// Lists prompts from queue
    Queue,

    /// Display GET request raw json output.
    Get {
        /// the route, e.g. "/history"
        route: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use Command::*;

    let args = Cli::parse();
    let client = Client::new(args.server, args.port);

    match args.command {
        Stats => {
            let stats: SystemStats = client.get("system_stats").await?;
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
        }
        Get { route } => {
            let response: serde_json::Value = client.get(route).await?;
            println!("{:#?}", response);
        }
        History => {
            let history: serde_json::Value = client.get("history").await?;
            println!("{:#?}", history);
        }
        Queue => {
            let queue: serde_json::Value = client.get("queue").await?;
            println!("{:#?}", queue);
        }
    }
    Ok(())
}
