mod commands;
mod io;

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser, Subcommand,
};
use cmfy::{Client, Result};
use commands::*;
use enum_dispatch::enum_dispatch;

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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let client = Client::from_hostname_port(args.hostname, args.port)?;
    args.command.run(client).await
}
