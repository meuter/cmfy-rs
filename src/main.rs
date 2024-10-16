use std::error::Error;
use clap::Parser;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub system: System,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub os: String,
    pub comfyui_version: String,
    pub python_version: String,
    pub pytorch_version: String,
    pub embedded_python: bool,
    pub argv: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub index: u32,
    pub vram_total: u64,
    pub vram_free: u64,
    pub torch_vram_total: u64,
    pub torch_vram_free: u64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();

    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/system_stats", args.server, args.port);
    let response = client.get(url).send().await.unwrap();
    let body = response.error_for_status()?.bytes().await?;
    let stats : SystemStats= serde_json::from_slice(&body)?;

    println!("{:#?}", stats);
    Ok(())
}
