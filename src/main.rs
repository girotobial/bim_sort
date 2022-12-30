mod cli;
#[warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
mod models;

use cli::Args;

use models::Bim;

#[allow(clippy::missing_errors_doc)]
fn main() -> std::io::Result<()> {
    let args = Args::get();

    let infile = args.in_file;

    let mut bim = Bim::from_file(&infile)?;

    bim.sort();

    bim.to_file(&infile)?;

    Ok(())
}
