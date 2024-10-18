mod client;
mod error;
mod status;

pub mod dto;
pub mod nodes;

pub use client::Client;
pub use dto::{History, Prompt, Queue, SystemStats};
pub use error::{Error, Result};
pub use status::{PromptAndStatus, Status};
