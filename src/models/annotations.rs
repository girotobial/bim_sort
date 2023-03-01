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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Annotation {
    pub name: String,
    pub value: String,
}

impl Ord for Annotation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Annotation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use crate::models::test::{there_and_back_test, FromValue};
    use serde_json::json;

    use super::*;

    #[test]
    #[should_panic]
    fn errors_if_field_is_unknown() {
        let bad_input = json!(
            {
                "name": "A name",
                "value": "A value",
                "UNKNOWN": "Bad field"
            }
        );

        Annotation::from_value(&bad_input);
    }

    #[test]
    fn values_can_be_arrays_or_strings() {
        let input_one = json!(
            {
                "name": "A new name",
                "value": [
                    "",
                    "An array value",
                    ""
                ]
            }
        );

        there_and_back_test(&input_one, Annotation::from_value);

        let input_two = json!(
            {
                "name": "Another name",
                "value": "A string value"
            }
        );

        there_and_back_test(&input_two, Annotation::from_value);
    }
}
