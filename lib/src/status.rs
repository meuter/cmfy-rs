use std::fmt::{Display, Formatter};

use colored::{ColoredString, Colorize};

#[derive(Debug, Clone)]
pub struct WithStatus<I, O> {
    pub inner: I,
    pub status: Status<O>,
}

#[derive(Debug, Clone)]
pub enum Status<O> {
    Completed(O),
    Pending,
    Running,
    Cancelled,
}

pub trait MarkAs {
    fn mark_as<O>(self, status: Status<O>) -> WithStatus<Self, O>
    where
        Self: Sized,
    {
        let inner = self;
        WithStatus { inner, status }
    }
}

impl<O> Display for Status<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Status::*;
        match self {
            Completed(_) => write!(f, "completed"),
            Pending => write!(f, "pending"),
            Running => write!(f, "running"),
            Cancelled => write!(f, "cancelled"),
        }
    }
}

impl<O> Status<O> {
    pub fn map<U, F>(self, f: F) -> Status<U>
    where
        F: Fn(O) -> U,
    {
        use Status::*;
        match self {
            Completed(output) => Completed(f(output)),
            Pending => Pending,
            Running => Running,
            Cancelled => Cancelled,
        }
    }

    pub fn colored(&self) -> ColoredString {
        match self {
            Status::Completed(_) => self.to_string().green(),
            Status::Pending => self.to_string().yellow(),
            Status::Running => self.to_string().blue(),
            Status::Cancelled => self.to_string().red(),
        }
    }
}
