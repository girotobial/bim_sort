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

use crate::models::expression::{Expression, Expressive};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Partition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_view: Option<String>,

    pub source: Source,
}

impl Ord for Partition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Partition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Source {
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Expression>,
}

impl Expressive for Source {
    fn expression(&self) -> Option<String> {
        self.expression.as_ref().map(Expression::to_string)
    }
}

#[cfg(test)]
mod test {
    use super::{Expression, Partition, Source};

    impl Partition {
        fn new(name: &str, dataview: &str, source: Source) -> Self {
            Self {
                mode: None,
                name: name.to_string(),
                data_view: Some(dataview.to_string()),
                source,
            }
        }
    }
    impl Source {
        fn new(type_: &str, expression: &str) -> Self {
            Self {
                type_: type_.to_string(),
                expression: Some(Expression::String(expression.to_string())),
            }
        }
    }

    #[test]
    fn test_can_sort_partitions() {
        let mut partitions = vec![
            Partition::new("2022 Onwards", "full", Source::new("m", "Some m script")),
            Partition::new("2020", "full", Source::new("m", "Some m script")),
        ];
        let expected = vec![
            Partition::new("2020", "full", Source::new("m", "Some m script")),
            Partition::new("2022 Onwards", "full", Source::new("m", "Some m script")),
        ];
        partitions.sort();
        assert_eq!(partitions, expected);
    }
}
