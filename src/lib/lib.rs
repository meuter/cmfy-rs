mod client;
mod error;

pub mod dto;
pub mod nodes;

pub use client::Client;
pub use dto::{History, Prompt, Queue, SystemStats};
pub use error::{Error, Result};
