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
use clap::{command, Parser};

#[derive(Parser)]
#[command(
    author,
    version,
    arg_required_else_help(true),
    about,
    after_help(LONG_ABOUT)
)]
pub struct Args {
    pub file: Option<std::path::PathBuf>,

    #[arg(short)]
    /// Print terms and conditions
    pub conditions: bool,

    #[arg(short)]
    /// Print warranty
    pub warranty: bool,
}

impl Args {
    #[must_use]
    pub fn get() -> Self {
        Self::parse()
    }
}

static LONG_ABOUT: &str = include_str!("about.txt");

#[allow(clippy::too_many_lines)]
//Outputs the warranty for the program
pub fn print_warranty() {
    const WARRANTY: &str = include_str!("warranty.txt");
    println!("{WARRANTY}");
}

#[allow(clippy::too_many_lines)]
pub fn print_tc() {
    const TERMS_AND_CONDITIONS: &str = include_str!("gpl.txt");

    println!("{TERMS_AND_CONDITIONS}");
}
