pub mod io;
pub mod models;
pub mod traits;

use std::str::FromStr;

use crate::traits::ColumnAttributes;
use io::{read_bim_file, write_bim_file};

fn main() -> io::Result<()> {
    let in_file = std::path::PathBuf::from_str("example.json").unwrap();
    let out_file = std::path::PathBuf::from_str("example_copy.json").unwrap();

    let bim = read_bim_file(&in_file)?;

    write_bim_file(&bim, &out_file)?;

    for table in bim.model.tables.iter() {
        println!("{}", table.name);
        for column in table.columns.iter() {
            println!("  |-{} | {}", column.name(), column.data_type());
        }
    }

    Ok(())
}
