use std::fmt::{Display, Formatter};

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
