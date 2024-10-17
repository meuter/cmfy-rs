use cmfy::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::io::Write;

pub use clio::{Input, Output};

pub trait JsonWrite {
    fn write_json(&mut self, value: &impl Serialize, pretty: bool) -> Result<()>;
}

impl JsonWrite for Output {
    fn write_json(&mut self, value: &impl Serialize, pretty: bool) -> Result<()> {
        let mut writer = self.lock();
        if pretty {
            serde_json::to_writer_pretty(&mut writer, &value)?;
        } else {
            serde_json::to_writer(&mut writer, &value)?;
        }
        writer.flush()?;
        Ok(())
    }
}

pub trait JsonRead {
    fn read_json<T: DeserializeOwned>(&mut self) -> Result<T>;
}

impl JsonRead for Input {
    fn read_json<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut read = self.lock();
        Ok(serde_json::from_reader(&mut read)?)
    }
}
