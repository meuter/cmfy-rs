mod client;
mod dto;
mod error;

pub use client::Client;
pub use dto::{History, Prompt, Queue, SystemStats};
pub use error::{Error, Result};
