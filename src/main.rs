use bim_sort::cli::Args;
use bim_sort::models::Bim;

#[allow(clippy::missing_errors_doc)]
fn main() -> std::io::Result<()> {
    let args = Args::get();

    let infile = args.in_file;

    let mut bim = Bim::from_file(&infile)?;

    bim.sort();

    bim.to_file(&infile)?;

    Ok(())
}
