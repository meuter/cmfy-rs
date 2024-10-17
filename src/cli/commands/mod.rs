mod cancel;
mod capture;
mod clear;
mod get;
mod history;
mod list;
mod open;
mod queue;
mod stats;
mod submit;

pub use cancel::Cancel;
pub use capture::Capture;
pub use clear::Clear;
pub use get::Get;
pub use history::History;
pub use list::List;
pub use open::Open;
pub use queue::Queue;
pub use stats::Stats;
pub use submit::Submit;

use cmfy::{Client, Result};

pub trait Run {
    async fn run(self, client: Client) -> Result<()>;
}
