mod client;
mod dto;
mod error;

use clap::Parser;
use client::Client;
use dto::SystemStats;
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let client = Client::new(args.server, args.port);
    let stats: SystemStats = client.get("system_stats").await?;
    println!("{:#?}", stats);
    Ok(())
}
