mod stats;
mod queue;
mod history;
mod get;
mod list;
mod capture;
mod submit;
mod open;

pub use stats::Stats;
pub use queue::Queue;
pub use history::History;
pub use get::Get;
pub use list::List;
pub use capture::Capture;
pub use submit::Submit;
pub use open::Open;

use cmfy::{Client, Result};

pub trait Run {
    async fn run(self, client: Client) -> Result<()>;
}
