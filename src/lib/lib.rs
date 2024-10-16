pub mod dto;
mod client;
mod error;

pub use client::Client;
pub use dto::{History, Prompt, Queue, SystemStats};
pub use error::{Error, Result};
