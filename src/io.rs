pub use io::Result;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde_json;

use crate::models::BimFile;

pub fn read_bim_file(path: &PathBuf) -> io::Result<BimFile> {
    let data = fs::read_to_string(path)?;
    let res: BimFile = serde_json::from_str(&data)?;
    Ok(res)
}

pub fn write_bim_file(data: &BimFile, path: &PathBuf) -> io::Result<()> {
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(path, contents)?;
    Ok(())
}
