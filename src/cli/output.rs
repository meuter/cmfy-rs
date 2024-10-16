use std::{fs::File, io::Write, path::Path};

use cmfy::{Error, Result};
use serde::Serialize;

pub struct Output(Box<dyn Write>);

impl<P> TryFrom<Option<P>> for Output
where
    P: AsRef<Path>,
{
    type Error = Error;

    fn try_from(maybe_path: Option<P>) -> Result<Self> {
        let writer: Box<dyn Write> = if let Some(path) = maybe_path {
            Box::new(File::create(path)?)
        } else {
            Box::new(std::io::stdout())
        };
        Ok(Self(writer))
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl Default for Output {
    fn default() -> Self {
        Self(Box::new(std::io::stdout()))
    }
}

impl Output {
    pub fn write_json<T: Serialize>(self, value: &T, pretty: bool) -> Result<()> {
        if pretty {
            Ok(serde_json::to_writer_pretty(self, &value)?)
        } else {
            Ok(serde_json::to_writer(self, &value)?)
        }
    }
}
