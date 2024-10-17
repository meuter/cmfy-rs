use cmfy::{Error, Result};
use serde::de::DeserializeOwned;
use std::{
    ffi::OsString,
    fmt::Display,
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

#[derive(Clone, Debug, Default)]
pub struct Input(Option<PathBuf>);

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.0 {
            write!(f, "{}", path.display())
        } else {
            write!(f, "<stdin>")
        }
    }
}

impl From<OsString> for Input {
    fn from(value: OsString) -> Self {
        Self(Some(PathBuf::from(value)))
    }
}

impl TryInto<Box<dyn Read>> for Input {
    type Error = Error;

    fn try_into(self) -> Result<Box<dyn Read>> {
        if let Some(path) = self.0 {
            Ok(Box::new(File::open(path)?))
        } else {
            Ok(Box::new(stdin()))
        }
    }
}

impl Input {
    pub fn read_json<T: DeserializeOwned>(self) -> Result<T> {
        let reader: Box<dyn Read> = self.try_into()?;
        Ok(serde_json::from_reader(reader)?)
    }
}
