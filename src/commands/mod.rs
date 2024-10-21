mod cancel;
mod capture;
mod clear;
mod download;
mod extract;
mod get;
mod history;
mod list;
mod listen;
mod monitor;
mod open;
mod queue;
mod stats;
mod submit;
mod view;

pub use cancel::Cancel;
pub use capture::Capture;
pub use clear::Clear;
pub use download::Download;
pub use extract::Extract;
pub use get::Get;
pub use history::History;
pub use list::List;
pub use listen::Listen;
pub use monitor::Monitor;
pub use open::Open;
pub use queue::Queue;
pub use stats::Stats;
pub use submit::Submit;
pub use view::View;

use cmfy::{Client, Result};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Run {
    async fn run(self, client: Client) -> Result<()>;
}
