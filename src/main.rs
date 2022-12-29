#[warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
pub mod models;
pub mod traits;

use models::Bim;
use std::str::FromStr;

const FILENAME: &str = "example.json";
const FILE2: &str = "example_changed.json";

#[allow(clippy::missing_errors_doc)]
fn main() -> std::io::Result<()> {
    let infile = std::path::PathBuf::from_str(FILENAME)
        .unwrap_or_else(|_| panic!("Could not find {}", FILENAME));

    let outfile =
        std::path::PathBuf::from_str(FILE2).unwrap_or_else(|_| panic!("Could not find {}", FILE2));

    let mut bim = Bim::from_file(&infile)?;

    bim.sort();

    bim.to_file(&outfile)?;

    Ok(())
}
