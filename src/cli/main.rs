mod commands;
mod io;

use clap::{Parser, Subcommand};
use cmfy::{Client, Result};
use commands::{Cancel, Capture, Get, History, List, Open, Queue, Run, Stats, Submit};

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
    Stats(Stats),
    History(History),
    Queue(Queue),
    List(List),
    Open(Open),
    Capture(Capture),
    Cancel(Cancel),
    Get(Get),
    Submit(Submit),
}

impl Run for Command {
    async fn run(self, client: Client) -> Result<()> {
        use Command::*;
        match self {
            Stats(cmd) => cmd.run(client).await,
            History(cmd) => cmd.run(client).await,
            Queue(cmd) => cmd.run(client).await,
            List(cmd) => cmd.run(client).await,
            Open(cmd) => cmd.run(client).await,
            Cancel(cmd) => cmd.run(client).await,
            Capture(cmd) => cmd.run(client).await,
            Get(cmd) => cmd.run(client).await,
            Submit(cmd) => cmd.run(client).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let client = Client::new(args.server, args.port);
    args.command.run(client).await
}
