mod list;

use clap::{Parser, Subcommand};
use cmfy::Client;
use humansize::{make_format, BINARY};
use list::PromptList;
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
    History {
        /// Clears all prompt from history after printing
        #[clap(long, short, action, default_value_t = false)]
        clear: bool,
    },

    /// Lists prompts from queue
    Queue,

    /// List all aprompts from history and queue
    List,

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
        History { clear } => {
            let history = client.history().await?;
            PromptList::from(history).display();
            if clear {
                let payload = serde_json::json!({"clear":true});
                client.post("history", &payload).await?;
            }
        }
        Queue => {
            let queue = client.queue().await?;
            PromptList::from(queue).display();
        }
        List => {
            let history = client.history().await?;
            let queue = client.queue().await?;
            PromptList::from((history, queue)).display();
        }
        Get { route } => {
            let response: serde_json::Value = client.get(route).await?;
            serde_json::to_writer(std::io::stdout(), &response)?;
        }
    }
    Ok(())
}
