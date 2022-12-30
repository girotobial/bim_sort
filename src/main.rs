/*
Bim Sort, sorts bim files for better compatibility with git
Copyright (C) 2022  Alexander Robinson

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use bim_sort::cli::{print_tc, print_warranty, Args};
use bim_sort::models::Bim;

#[allow(clippy::missing_errors_doc)]
fn main() -> std::io::Result<()> {
    let args = Args::get();

    if args.conditions {
        print_tc();
        return Ok(());
    }

    if args.warranty {
        print_warranty();
        return Ok(());
    }

    if let Some(infile) = args.file {
        let mut bim = Bim::from_file(&infile)?;

        bim.sort();

        bim.to_file(&infile)?;
    }
    Ok(())
}
