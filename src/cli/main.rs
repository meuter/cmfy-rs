mod commands;

use clap::{Parser, Subcommand};
use cmfy::{Client, Result};
use commands::{Get, History, List, Queue, Run, Stats};

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
    Get(Get),
}

impl Run for Command {
    async fn run(self, client: Client) -> Result<()> {
        use Command::*;
        match self {
            Stats(stats) => stats.run(client).await,
            History(history) => history.run(client).await,
            Queue(queue) => queue.run(client).await,
            List(list) => list.run(client).await,
            Get(get) => get.run(client).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let client = Client::new(args.server, args.port);
    args.command.run(client).await
}
