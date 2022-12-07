#[warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
pub mod io;
pub mod models;
pub mod traits;

use std::str::FromStr;

use io::read_bim_file;

const FILENAME: &str = "example.json";

#[allow(clippy::missing_errors_doc)]
fn main() -> io::Result<()> {
    let left_file = std::path::PathBuf::from_str(FILENAME)
        .unwrap_or_else(|_| panic!("Could not find {}", FILENAME));
    let right_file = std::path::PathBuf::from_str(FILENAME)
        .unwrap_or_else(|_| panic!("Could not find {}", FILENAME));

    let left_bim = read_bim_file(&left_file)?;
    let right_bim = read_bim_file(&right_file)?;

    let does_match = left_bim == right_bim;

    print!("{}", does_match);

    Ok(())
}
