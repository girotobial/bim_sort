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

use crate::models::expression::Expression;
use crate::models::traits::RecursiveSort;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CalculationItem {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Expression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    format_string_definition: Option<FormatStringDefinition>,
}

impl PartialOrd for CalculationItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CalculationItem {
    fn cmp_ordinal(&self, other: &Self) -> std::cmp::Ordering {
        match (self.ordinal, other.ordinal) {
            (Some(s), Some(o)) => s.cmp(&o),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    }

    fn cmp_name(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Ord for CalculationItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        let ordinal_cmp = self.cmp_ordinal(other);
        match ordinal_cmp {
            Equal => self.cmp_name(other),
            _ => ordinal_cmp,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct FormatStringDefinition {
    expression: Expression,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CalculationGroup {
    calculation_items: Vec<CalculationItem>,
}

impl RecursiveSort for CalculationGroup {
    fn recursive_sort(&mut self) {
        self.calculation_items.sort();
    }
}

#[cfg(test)]
mod test {
    use super::CalculationItem;

    use crate::models::test::{there_and_back_test, FromValue};

    #[test]
    fn test_calculation_item_has_format_string_definition() {
        let data = serde_json::json!(
            {
                "name": "item with calculation",
                "formatStringDefinition": {
                    "expression": "\"0.0%\""
                }
            }
        );

        there_and_back_test(&data, CalculationItem::from_value);
    }
}
