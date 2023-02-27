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

use super::skip_if::{is_true, true_};

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Relationship {
    pub name: String,
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,

    #[serde(default = "true_", skip_serializing_if = "is_true")]
    pub is_active: bool,

    #[serde(
        default = "CrossFilterBehaviour::default",
        skip_serializing_if = "CrossFilterBehaviour::is_single"
    )]
    cross_filter_behaviour: CrossFilterBehaviour,
}

impl Ord for Relationship {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Relationship {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Eq)]
#[allow(clippy::use_self)] // False positive
pub enum CrossFilterBehaviour {
    Single,

    #[serde(rename = "bothDirections")]
    Both,
}

impl CrossFilterBehaviour {
    #[must_use]
    pub const fn default() -> Self {
        Self::Single
    }

    #[must_use]
    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single)
    }
}

#[cfg(test)]
mod test {
    use crate::models::relationship::CrossFilterBehaviour;

    use super::Relationship;
    use serde_json;

    #[test]
    fn serialize_active_relationship() {
        let input = r#"{
            "name": "fd4c11a5-2d37-4e9f-a17c-3bdb83ace28d",
            "fromTable": "Table1",
            "fromColumn": "id",
            "toTable": "Table2",
            "toColumn": "table1_id"
        }
        "#;

        let relationship: Relationship = serde_json::from_str(input).unwrap();

        assert_eq!(relationship.name, "fd4c11a5-2d37-4e9f-a17c-3bdb83ace28d");
        assert_eq!(relationship.from_table, "Table1");
        assert_eq!(relationship.from_column, "id");
        assert_eq!(relationship.to_table, "Table2");
        assert_eq!(relationship.to_column, "table1_id");
        assert!(relationship.is_active);
        assert_eq!(
            relationship.cross_filter_behaviour,
            CrossFilterBehaviour::Single
        );
    }

    #[test]
    fn serialize_inactive_relationship() {
        let input = r#"
        {
            "name": "fd4c11a5-2d37-4e9f-a17c-3bdb83ace28d",
            "fromTable": "Table1",
            "fromColumn": "id",
            "toTable": "Table2",
            "toColumn": "table1_id",
            "isActive": false
        }"#;

        let relationship: Relationship = serde_json::from_str(input).unwrap();

        assert_eq!(relationship.name, "fd4c11a5-2d37-4e9f-a17c-3bdb83ace28d");
        assert_eq!(relationship.from_table, "Table1");
        assert_eq!(relationship.from_column, "id");
        assert_eq!(relationship.to_table, "Table2");
        assert_eq!(relationship.to_column, "table1_id");
        assert!(!relationship.is_active);
    }

    #[test]
    fn serialize_both_direction_relationship() {
        let input = r#"{
            "name": "fd4c11a5-2d37-4e9f-a17c-3bdb83ace28d",
            "fromTable": "Table1",
            "fromColumn": "id",
            "toTable": "Table2",
            "toColumn": "table1_id",
            "crossFilterBehaviour": "bothDirections"
        }
        "#;

        let relationship: Relationship = serde_json::from_str(input).unwrap();

        assert_eq!(
            relationship.cross_filter_behaviour,
            CrossFilterBehaviour::Both
        );
    }
}
