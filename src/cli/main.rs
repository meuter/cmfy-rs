use clap::{Parser, Subcommand};
use cmfy::{Client, Prompt};
use humansize::{make_format, BINARY};
use itertools::Itertools;
use std::{error::Error, iter::empty};

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

struct PromptListEntry {
    prompt: Prompt,
    status: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use Command::*;

    let args = Cli::parse();
    let client = Client::new(args.server, args.port);

    match args.command {
        Stats => {
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
        }
        Get { route } => {
            let response: serde_json::Value = client.get(route).await?;
            println!("{:#?}", response);
        }
        History => {
            let history = client.history().await?;
            let mut entries = history
                .into_values()
                .map(|entry| {
                    let status = if entry.cancelled() { "cancelled" } else { "completed" };
                    let prompt = entry.prompt;
                    PromptListEntry { prompt, status }
                })
                .collect_vec();
            entries.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
            for entry in entries {
                let prompt = entry.prompt;
                let index = format!("[{}]", prompt.index);
                println!("{:<5}{} ({})", index, prompt.uuid, entry.status);
            }
        }
        Queue => {
            let queue = client.queue().await?;
            let mut entries = empty()
                .chain(queue.running.into_iter().map(|prompt| PromptListEntry {
                    prompt,
                    status: "running",
                }))
                .chain(queue.pending.into_iter().map(|prompt| PromptListEntry {
                    prompt,
                    status: "pending",
                }))
                .collect_vec();
            entries.sort_by(|l, r| l.prompt.index.cmp(&r.prompt.index));
            for entry in entries {
                let prompt = entry.prompt;
                let index = format!("[{}]", prompt.index);
                println!("{:<5}{} ({})", index, prompt.uuid, entry.status);
            }
        }
    }
    Ok(())
}
