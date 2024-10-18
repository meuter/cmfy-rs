use colored::Colorize;

use crate::{dto::Outputs, Prompt};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct PromptAndStatus {
    pub prompt: Prompt,
    pub status: Status<Outputs>,
}

#[derive(Debug, Clone)]
pub enum Status<O> {
    Completed(O),
    Pending,
    Running,
    Cancelled,
}

impl<O> Display for Status<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Status::*;
        match self {
            Completed(_) => write!(f, "{}", "completed".green()),
            Pending => write!(f, "{}", "pending".yellow()),
            Running => write!(f, "{}", "running".blue()),
            Cancelled => write!(f, "{}", "cancelled".red()),
        }
    }
}

impl PromptAndStatus {
    pub fn running(prompt: Prompt) -> Self {
        let status = Status::Running;
        Self { prompt, status }
    }

    pub fn pending(prompt: Prompt) -> Self {
        let status = Status::Pending;
        Self { prompt, status }
    }

    pub fn cancelled(prompt: Prompt) -> Self {
        let status = Status::Cancelled;
        Self { prompt, status }
    }

    pub fn completed(prompt: Prompt, outputs: Outputs) -> Self {
        let status = Status::Completed(outputs);
        Self { prompt, status }
    }
}
