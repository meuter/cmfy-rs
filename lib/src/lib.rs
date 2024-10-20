mod client;
mod error;
mod status;
mod websocket;

pub mod dto;

pub use client::Client;
pub use dto::{History, Prompt, Queue, SystemStats};
pub use error::{Error, Result};
pub use status::{MarkAs, Status, WithStatus};
pub use websocket::MessageStream;
