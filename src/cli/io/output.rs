use cmfy::{Error, Result};
use serde::Serialize;
use std::{
    ffi::OsString,
    fmt::Display,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};

#[derive(Clone, Debug, Default)]
pub struct Output(Option<PathBuf>);

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.0 {
            write!(f, "{}", path.display())
        } else {
            write!(f, "<stdout>")
        }
    }
}

impl From<OsString> for Output {
    fn from(value: OsString) -> Self {
        if value == "<stdout>" {
            Self::default()
        } else {
            Self(Some(PathBuf::from(value)))
        }
    }
}

impl TryInto<Box<dyn Write>> for Output {
    type Error = Error;

    fn try_into(self) -> Result<Box<dyn Write>> {
        if let Some(path) = self.0 {
            Ok(Box::new(File::create(path)?))
        } else {
            Ok(Box::new(stdout()))
        }
    }
}

impl Output {
    pub fn write_json(self, value: &impl Serialize, pretty: bool) -> Result<()> {
        let writer: Box<dyn Write> = self.try_into()?;
        if pretty {
            Ok(serde_json::to_writer_pretty(writer, &value)?)
        } else {
            Ok(serde_json::to_writer(writer, &value)?)
        }
    }
}
