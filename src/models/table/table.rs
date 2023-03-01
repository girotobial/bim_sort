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

use crate::models::skip_if::{false_, is_false};
use crate::models::traits::RecursiveSort;

use super::calculation_group::CalculationGroup;
use super::Column;
use super::Measure;
use super::Partition;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Table {
    pub name: String,

    #[serde(default = "false_", skip_serializing_if = "is_false")]
    pub is_hidden: bool,
    pub columns: Vec<Column>,
    pub partitions: Vec<Partition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<Measure>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calculation_group: Option<CalculationGroup>,
}

impl RecursiveSort for Table {
    fn recursive_sort(&mut self) {
        self.partitions.sort();
        self.columns.sort();
        if let Some(v) = &mut self.measures {
            v.recursive_sort();
        }

        if let Some(c) = &mut self.calculation_group {
            c.recursive_sort();
        }
    }
}

impl PartialOrd for Table {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Table {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::{CalculationGroup, CalculationItem, Table};
    use crate::models::test::{there_and_back_test, FromValue};
    use serde_json::json;

    #[test]
    fn can_build_calculation_item_from_json() {
        let input = json!(
            {
                "name": "Next Day",
                "expression": [
                    "CALCULATE (",
                    "    SELECTEDMEASURE (),",
                    "    FILTER ( CourseDate, CourseDate[day_offset] <= 1 )",
                    ")"
                ],
                "ordinal": 0
            }
        );

        there_and_back_test(&input, CalculationItem::from_value);
    }

    #[test]
    #[should_panic]
    fn calculation_item_fails_without_a_name() {
        let input = json!(
            {
                "expression": [
                    "CALCULATE (",
                    "    SELECTEDMEASURE (),",
                    "    FILTER ( CourseDate, CourseDate[day_offset] <= 1 )",
                    ")"
                ],
                "ordinal": 0
            }
        );

        CalculationItem::from_value(&input);
    }

    #[test]
    fn calculation_item_succeeds_without_an_expression() {
        let input = json!(
            {
                "name": "Next Day",
                "ordinal": 0
            }
        );

        there_and_back_test(&input, CalculationItem::from_value);
    }

    #[test]
    fn calculation_item_succeeds_without_an_ordinal() {
        let input = json!(
            {
                "name": "Next Day",
                "expression": [
                    "CALCULATE (",
                    "    SELECTEDMEASURE (),",
                    "    FILTER ( CourseDate, CourseDate[day_offset] <= 1 )",
                    ")"
                ]
            }
        );

        there_and_back_test(&input, CalculationItem::from_value);
    }

    #[test]
    fn calculation_item_succeeds_with_name_only() {
        let input = json!(
            {
                "name": "CalculationItem 1"
            }
        );

        there_and_back_test(&input, CalculationItem::from_value);
    }

    #[test]
    fn can_create_calculation_group_from_json() {
        let input = json!(
            {
                "calculationItems": [
                    {
                        "name": "Next Day",
                        "expression": [
                            "CALCULATE (",
                            "    SELECTEDMEASURE (),",
                            "    FILTER ( CourseDate, CourseDate[day_offset] <= 1 )",
                            ")"
                        ],
                        "ordinal": 1
                    }
                ]
            }
        );

        there_and_back_test(&input, CalculationGroup::from_value)
    }

    #[test]
    fn can_create_calculation_group_with_empty_calculation_items() {
        let input = json!(
            {
                "calculationItems": []
            }
        );
        there_and_back_test(&input, CalculationGroup::from_value);
    }

    fn test_sort<T: Ord + std::fmt::Debug, F>(inputs: [serde_json::Value; 2], f: F)
    where
        F: Fn(&serde_json::Value) -> T,
    {
        use std::rc::Rc;
        let item_one = Rc::new(f(&inputs[0]));
        let item_two = Rc::new(f(&inputs[1]));

        let mut items = [Rc::clone(&item_one), Rc::clone(&item_two)];
        let expected = [item_two, item_one];

        items.sort();

        assert_eq!(items, expected);
    }

    #[test]
    fn calculation_items_can_sort_by_name() {
        let input_one = json!(
            {
                "name": "Next Day",
            }
        );
        let input_two = json!(
            {
                "name": "Yesterday"
            }
        );

        test_sort([input_two, input_one], CalculationItem::from_value);
    }

    #[test]
    fn calculation_items_prioritise_sort_by_ordinal() {
        let input_one = json!(
            {
                "name": "Next Day",
                "ordinal": 1
            }
        );
        let input_two = json!(
            {
                "name": "Yesterday",
                "ordinal": 0
            }
        );

        test_sort([input_one, input_two], CalculationItem::from_value);
    }

    #[test]
    fn calculation_items_sort_by_name_if_ordinal_equal() {
        let input_one = json!(
            {
                "name": "Next Day",
                "ordinal": 0
            }
        );
        let input_two = json!(
            {
                "name": "Yesterday",
                "ordinal": 0
            }
        );

        test_sort([input_two, input_one], CalculationItem::from_value);
    }

    #[test]
    fn calculation_items_sort_ordinals_first_if_other_only_has_name() {
        let input_one = json!(
            {
                "name": "Next Day",
            }
        );
        let input_two = json!(
            {
                "name": "Yesterday",
                "ordinal": 0
            }
        );

        test_sort([input_one, input_two], CalculationItem::from_value);
    }

    #[test]
    fn can_create_table_with_a_calculation_group() {
        let input = json!(
            {
                "name": "CalculationGroup 1",
                "calculationGroup": {
                    "calculationItems": [
                        {
                            "name": "CalculationItem 1"
                        }
                    ]
                },
                "columns": [
                    {
                        "name": "CalculationItemColumn 1",
                        "dataType": "string",
                        "sourceColumn": "Name"
                    }
                ],
                "partitions": [
                    {
                        "name": "CalculationGroup 1",
                        "mode": "import",
                        "source": {
                            "type": "calculationGroup"
                        }
                    }
                ]
            }
        );

        there_and_back_test(&input, Table::from_value);
    }

    #[test]
    fn can_create_table_without_calculation_group() {
        let input = json!(
            {
                "name": "arrival_time",
                "columns": [
                    {
                        "name": "time",
                        "dataType": "int64",
                        "isHidden": true,
                        "sourceColumn": "time"
                    },
                    {
                        "name": "MinutesInDay",
                        "dataType": "int64",
                        "isHidden": true,
                        "sourceColumn": "MinutesInDay"
                    },
                    {
                        "name": "DayNum",
                        "dataType": "int64",
                        "sourceColumn": "DayNum"
                    },
                    {
                        "name": "Hour",
                        "dataType": "int64",
                        "sourceColumn": "Hour"
                    },
                    {
                        "name": "Minutes",
                        "dataType": "int64",
                        "sourceColumn": "Minutes"
                    },
                    {
                        "name": "Timestamp",
                        "dataType": "dateTime",
                        "sourceColumn": "Timestamp"
                    }
                ],
                "partitions": [
                    {
                        "name": "Partition",
                        "dataView": "full",
                        "source": {
                            "type": "m",
                            "expression": [
                                "let",
                                "    Source = #\"times ref\"",
                                "in",
                                "    Source"
                            ]
                        }
                    }
                ]
            }
        );

        there_and_back_test(&input, Table::from_value);
    }
}
