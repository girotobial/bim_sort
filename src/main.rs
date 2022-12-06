pub mod io;
pub mod models;
pub mod traits;

use std::str::FromStr;

use io::read_bim_file;

fn main() -> io::Result<()> {
    let left_file = std::path::PathBuf::from_str("example.json").unwrap();
    let right_file = std::path::PathBuf::from_str("example.json").unwrap();

    let left_bim = read_bim_file(&left_file)?;
    let right_bim = read_bim_file(&right_file)?;

    let does_match = left_bim == right_bim;

    print!("{}", does_match);

    Ok(())
}
