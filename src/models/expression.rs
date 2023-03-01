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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum Expression {
    Vec(Vec<String>),
    String(String),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Vec(v) => v.join("\n"),
            Self::String(s) => s.clone(),
        };
        write!(f, "{}", output)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

pub trait Expressive {
    fn expression(&self) -> Option<String>;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModelExpression {
    pub name: String,

    pub kind: String,
    expression: Expression,
}

impl Ord for ModelExpression {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for ModelExpression {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Expressive for ModelExpression {
    #[must_use]
    fn expression(&self) -> Option<String> {
        Some(self.expression.to_string())
    }
}
