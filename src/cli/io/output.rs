use cmfy::{Error, Result};
use serde::Serialize;
use std::{
    ffi::OsString,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
};

#[derive(Clone, Debug, Default)]
pub struct Output(Option<PathBuf>);

impl From<OsString> for Output {
    fn from(value: OsString) -> Self {
        Self(Some(PathBuf::from(value)))
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
