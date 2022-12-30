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

pub use io::Result;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Model;

/// The root bim file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bim {
    pub name: String,
    pub compatibility_level: u32,
    pub model: Model,
    pub id: String,
}

impl Bim {
    pub fn from_file(path: &PathBuf) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let res: Self = serde_json::from_str(&data)?;
        Ok(res)
    }

    pub fn to_file(&self, path: &PathBuf) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn sort(&mut self) {
        use super::traits::RecursiveSort;
        self.model.recursive_sort();
    }
}
