use serde_json;
use std::io;

use crate::models::BimFile;
pub use io::Result;

use std::fs;

pub fn read_bim_file(path: &str) -> io::Result<BimFile> {
    let data = fs::read_to_string(path)?;
    let res: BimFile = serde_json::from_str(&data)?;
    Ok(res)
}

pub fn write_bim_file(data: &BimFile, path: &str) -> io::Result<()> {
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(path, contents)?;
    Ok(())
}
