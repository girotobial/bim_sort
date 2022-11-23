pub mod io;
pub mod models;
pub mod traits;

use std::str::FromStr;

use io::{read_bim_file, write_bim_file};

fn main() -> io::Result<()> {
    let in_file = std::path::PathBuf::from_str("example.json").unwrap();
    let out_file = std::path::PathBuf::from_str("datasources_new.json").unwrap();

    let res = read_bim_file(&in_file)?;

    write_bim_file(&res, &out_file)?;

    println!("{:?}", res);

    Ok(())
}
