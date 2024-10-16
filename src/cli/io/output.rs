use cmfy::{Error, Result};
use serde::Serialize;
use std::{fs::File, io::Write, path::PathBuf};

pub struct Output(Box<dyn Write>);

impl Default for Output {
    fn default() -> Self {
        Self(Box::new(std::io::stdout()))
    }
}

impl TryFrom<PathBuf> for Output {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        Ok(Self(Box::new(File::create(path)?)))
    }
}

impl TryFrom<Option<PathBuf>> for Output {
    type Error = Error;

    fn try_from(maybe_path: Option<PathBuf>) -> Result<Self> {
        if let Some(path) = maybe_path {
            Self::try_from(path)
        } else {
            Ok(Self::default())
        }
    }
}

impl Output {
    pub fn write_json(self, value: &impl Serialize, pretty: bool) -> Result<()> {
        if pretty {
            Ok(serde_json::to_writer_pretty(self.0, &value)?)
        } else {
            Ok(serde_json::to_writer(self.0, &value)?)
        }
    }
}
