use std::error::Error;
use clap::Parser;

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
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();

    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/system_stats", args.server, args.port);
    let response = client.get(url).send().await.unwrap();
    let body = response.error_for_status()?.bytes().await;

    println!("{:#?}", body);
    Ok(())
}
