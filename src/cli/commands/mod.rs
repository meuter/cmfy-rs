mod stats;
mod queue;
mod history;
mod get;
mod list;

pub use stats::Stats;
pub use queue::Queue;
pub use history::History;
pub use get::Get;
pub use list::List;

use cmfy::{Client, Result};

pub trait Run {
    async fn run(self, client: Client) -> Result<()>;
}
