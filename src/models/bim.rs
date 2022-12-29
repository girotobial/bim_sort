pub use io::Result;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bim {
    pub name: String,
    pub compatibility_level: u32,
    pub model: Model,
    pub id: String,
}

impl Bim {
    pub fn from_file(path: &PathBuf) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let res: Bim = serde_json::from_str(&data)?;
        Ok(res)
    }

    pub fn to_file(self: &Self, path: &PathBuf) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}
