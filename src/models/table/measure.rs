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

use crate::models::annotations::Annotation;
use crate::models::expression::{Expression, Expressive};
use crate::models::RecursiveSort;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Measure {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    expression: Expression,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_folder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
}

impl Ord for Measure {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Measure {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Expressive for Measure {
    fn expression(&self) -> Option<String> {
        Some(self.expression.to_string())
    }
}

impl RecursiveSort for Measure {
    fn recursive_sort(&mut self) {
        if let Some(a) = &mut self.annotations {
            a.sort()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Expression;
    use super::Measure;
    use super::RecursiveSort;
    use crate::models::test::{there_and_back_test, FromValue};

    impl Measure {
        fn new(name: &str, expression: &str) -> Self {
            let expression = Expression::String(expression.to_string());
            Self {
                name: name.to_string(),
                description: None,
                expression,
                format_string: None,
                display_folder: None,
                annotations: None,
            }
        }
    }

    #[test]
    fn test_can_sort_measures() {
        let mut measures = vec![
            Measure::new("Total Count", "COUNTROWS(Table)"),
            Measure::new(
                "Days Per Month",
                "AVERAGEX ( VALUES ( 'Calendar'[MonthStarting] ), [Number of Days Delivered] )",
            ),
        ];

        let expected = vec![
            Measure::new(
                "Days Per Month",
                "AVERAGEX ( VALUES ( 'Calendar'[MonthStarting] ), [Number of Days Delivered] )",
            ),
            Measure::new("Total Count", "COUNTROWS(Table)"),
        ];

        measures.recursive_sort();
        assert_eq!(measures, expected);
    }

    #[test]
    fn test_measures_allow_annotations() {
        let input = serde_json::json!(
            {
                "name": "Date From",
                "expression": " MIN('Calendar'[Date])",
                "formatString": "dd/MM/yyyy",
                "displayFolder": "Filters",
                "annotations": [
                    {
                        "name": "Format",
                        "value": "<Format Format=\"DateTimeCustom\"><DateTimes><DateTime LCID=\"2057\" Group=\"ShortDate\" FormatString=\"dd/MM/yyyy\" /></DateTimes></Format>"
                    }
                ]
            }
        );

        there_and_back_test(&input, Measure::from_value)
    }

    #[test]
    fn test_measures_can_have_descriptions() {
        let input = serde_json::json!(
            {
                "name": "A measure",
                "description": "A measure's description",
                "expression": [
                    "",
                    "COUNTROWS(Table)",
                    ""
                ],
            }
        );

        there_and_back_test(&input, Measure::from_value);
    }
}
