use cmfy::{Error, Result};
use serde::de::DeserializeOwned;
use std::{fs::File, io::Read, path::PathBuf};

pub struct Input(Box<dyn Read>);

impl Default for Input {
    fn default() -> Self {
        Self(Box::new(std::io::stdin()))
    }
}

impl TryFrom<PathBuf> for Input {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        Ok(Self(Box::new(File::open(path)?)))
    }
}

impl TryFrom<Option<PathBuf>> for Input {
    type Error = Error;

    fn try_from(maybe_path: Option<PathBuf>) -> Result<Self> {
        if let Some(path) = maybe_path {
            Self::try_from(path)
        } else {
            Ok(Self::default())
        }
    }
}

impl Input {
    pub fn read_json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_reader(self.0)?)
    }
}
