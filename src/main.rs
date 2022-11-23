pub mod io;
pub mod models;

use io::{read_bim_file, write_bim_file};

fn main() -> io::Result<()> {
    let path = "example.json";
    let res = read_bim_file(path)?;

    write_bim_file(&res, "datasources_new.json")?;

    println!("{:?}", res);

    Ok(())
}
