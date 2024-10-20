mod commands;
mod io;

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser, Subcommand,
};
use cmfy::{Client, Result};
use commands::*;
use enum_dispatch::enum_dispatch;
use ring::digest::{digest, SHA256};
use uuid::Uuid;

pub fn build_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().underline())
        .usage(AnsiColor::Yellow.on_default().underline())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::BrightWhite.on_default())
        .error(AnsiColor::BrightRed.on_default())
}

#[derive(Parser, Debug)]
#[clap(version)]
#[command(styles=build_styles(), color=clap::ColorChoice::Always, infer_subcommands = true)]
struct Cli {
    /// hostname of the server
    #[arg(
        short = 's',
        long,
        env = "COMFY_HOSTNAME",
        value_name = "HOSTNAME",
        default_value = "localhost"
    )]
    hostname: String,

    /// port of the server
    #[arg(short, long, env = "COMFY_PORT", value_name = "PORT", default_value_t = 8188)]
    port: u32,

    /// client id advertised to the server, allows to spoof an existing client
    /// when listening to websocket message. If none is provided, a unique id
    /// will be computed.
    #[arg(short, long, env = "COMFY_CLIENT_ID", value_name = "CLIENT_ID")]
    client_id: Option<String>,

    /// command to execute
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(Run)]
enum Command {
    Stats(Stats),
    History(History),
    Queue(Queue),
    List(List),
    Cancel(Cancel),
    Clear(Clear),
    Open(Open),
    Capture(Capture),
    Submit(Submit),
    View(View),
    // TODO: add a post?
    Get(Get),
    Listen(Listen),
    Extract(Extract),
}

fn compute_own_client_id() -> String {
    let hash_input = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let hash = digest(&SHA256, hash_input.as_bytes());
    let client_id = Uuid::from_slice(&hash.as_ref()[0..16]).unwrap();
    client_id.to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let client_id = args.client_id.unwrap_or_else(compute_own_client_id);
    let client = Client::new(args.hostname, args.port, client_id);
    args.command.run(client).await
}
