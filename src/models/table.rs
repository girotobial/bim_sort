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

pub use self::column::{Attributes, Column};
pub use self::partition::{Partition, Source};
use super::annotations::Annotation;
use super::expression::{Expression, Expressive};

use self::measure::Measure;
use super::skip_if::{false_, is_false};
use super::traits::RecursiveSort;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Table {
    pub name: String,

    #[serde(default = "false_", skip_serializing_if = "is_false")]
    pub is_hidden: bool,
    pub columns: Vec<column::Column>,
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
            v.sort();
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

mod column {
    use super::Annotation;
    use super::RecursiveSort;
    use super::{Expression, Expressive};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged, rename_all = "camelCase", deny_unknown_fields)]
    pub enum Column {
        Calculated(Calculated),
        Sourced(Sourced),
    }

    impl PartialOrd for Column {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Column {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.name().to_lowercase().cmp(&other.name().to_lowercase())
        }
    }

    impl Attributes for Column {
        fn name(&self) -> String {
            match self {
                Self::Calculated(c) => c.name(),
                Self::Sourced(c) => c.name(),
            }
        }
        fn data_type(&self) -> String {
            match self {
                Self::Calculated(c) => c.data_type(),
                Self::Sourced(c) => c.data_type(),
            }
        }
        fn is_hidden(&self) -> bool {
            match self {
                Self::Calculated(c) => c.is_hidden(),
                Self::Sourced(c) => c.is_hidden(),
            }
        }
    }

    impl Expressive for Column {
        fn expression(&self) -> Option<String> {
            match self {
                Self::Calculated(c) => c.expression(),
                Self::Sourced(_) => None,
            }
        }
    }

    impl RecursiveSort for Column {
        fn recursive_sort(&mut self) {
            if let Self::Sourced(s) = self {
                if let Some(a) = &mut s.annotations {
                    a.sort();
                }
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct CommonColumn {
        name: String,
        data_type: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        is_hidden: Option<bool>,
    }

    pub trait Attributes {
        fn name(&self) -> String;
        fn data_type(&self) -> String;
        fn is_hidden(&self) -> bool;
    }

    impl Attributes for CommonColumn {
        fn name(&self) -> String {
            self.name.clone()
        }
        fn data_type(&self) -> String {
            self.data_type.clone()
        }
        fn is_hidden(&self) -> bool {
            self.is_hidden.unwrap_or(false)
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct Calculated {
        #[serde(flatten)]
        common: CommonColumn,

        #[serde(rename = "type")]
        pub type_: String,
        expression: Expression,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub is_data_type_inferred: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_string: Option<String>,
        pub display_folder: Option<String>,
    }

    impl Expressive for Calculated {
        fn expression(&self) -> Option<String> {
            Some(self.expression.to_string())
        }
    }

    impl Attributes for Calculated {
        fn name(&self) -> String {
            self.common.name()
        }
        fn data_type(&self) -> String {
            self.common.data_type()
        }
        fn is_hidden(&self) -> bool {
            self.common.is_hidden()
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct Sourced {
        #[serde(flatten)]
        common: CommonColumn,
        pub source_column: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_string: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub annotations: Option<Vec<Annotation>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub sort_by_column: Option<String>,
    }

    impl Attributes for Sourced {
        fn data_type(&self) -> String {
            self.common.data_type()
        }
        fn is_hidden(&self) -> bool {
            self.common.is_hidden()
        }
        fn name(&self) -> String {
            self.common.name()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Column;
        use super::Expressive;
        use serde_json;

        #[test]
        fn test_column_with_vec_expression() {
            let column_content = r#"
            {
                "type": "calculated",
                "name": "Column name",
                "dataType": "int64",
                "isDataTypeInferred": true,
                "expression": [
                    "",
                    "VAR some_val =",
                    "    COUNTROWS( FILTER( Table, Table[DontCountMe] ) )",
                    "VAR second_val =",
                    "    COUNTROWS( Table )",
                    "RETURN",
                    "    DIVIDE( some_val, second_val )"
                ],
                "formatString": "0.0%;-0.0%;0.0%",
                "displayFolder": "MyMeasures"
            }"#;
            let expected_expression = [
                "",
                "VAR some_val =",
                "    COUNTROWS( FILTER( Table, Table[DontCountMe] ) )",
                "VAR second_val =",
                "    COUNTROWS( Table )",
                "RETURN",
                "    DIVIDE( some_val, second_val )",
            ]
            .join("\n");

            let column: Column = serde_json::from_str(column_content).expect("Should not fail");
            assert_eq!(
                column.expression().expect("Should not fail"),
                expected_expression
            );
        }

        use super::Calculated;
        use super::CommonColumn;
        use super::Sourced;

        impl Column {
            fn new_calculated(name: &str, data_type: &str, expression: &str) -> Self {
                Self::Calculated(Calculated {
                    common: CommonColumn {
                        name: name.to_string(),
                        data_type: data_type.to_string(),
                        is_hidden: None,
                    },
                    type_: "calculated".to_string(),
                    expression: crate::models::Expression::String(expression.to_string()),
                    is_data_type_inferred: None,
                    format_string: None,
                    display_folder: None,
                })
            }
            fn new_sourced(
                name: &str,
                data_type: &str,
                source_column: &str,
                sort_by_column: &str,
            ) -> Self {
                Self::Sourced(Sourced {
                    common: CommonColumn {
                        name: name.to_string(),
                        data_type: data_type.to_string(),
                        is_hidden: None,
                    },
                    source_column: source_column.to_string(),
                    sort_by_column: Some(sort_by_column.to_string()),
                    description: None,
                    format_string: None,
                    annotations: None,
                })
            }
        }

        #[test]
        fn test_can_sort_columns() {
            let mut columns = vec![
                Column::new_calculated("ZZZ Calculated", "int64", "COUNTROWS(Calculations)"),
                Column::new_sourced("ZZZ Sourced", "int64", "ZZZ Sourced", "ZZZ Sourced"),
                Column::new_sourced("AAA Sourced", "int64", "AAA Sourced", "AAA Sourced"),
                Column::new_calculated("AAA Calculated", "int64", "COUNTROWS(Calculated)"),
            ];
            let expected = vec![
                Column::new_calculated("AAA Calculated", "int64", "COUNTROWS(Calculated)"),
                Column::new_sourced("AAA Sourced", "int64", "AAA Sourced", "AAA Sourced"),
                Column::new_calculated("ZZZ Calculated", "int64", "COUNTROWS(Calculations)"),
                Column::new_sourced("ZZZ Sourced", "int64", "ZZZ Sourced", "ZZZ Sourced"),
            ];

            columns.sort();
            assert_eq!(columns, expected);
        }

        #[test]
        fn test_if_source_column_is_provided_it_should_be_outputted() {
            let input = r#"
                {
                    "name": "Time Horizon",
                    "dataType": "string",
                    "sourceColumn": "Name",
                    "sortByColumn": "Ordinal"
                }
            "#;

            let column: Column = serde_json::from_str(input).unwrap();

            let output = serde_json::to_string(&column).unwrap();

            assert!(output.contains(r#""sourceColumn":"Name""#))
        }

        #[test]
        fn test_if_sort_by_column_provided_it_is_in_output() {
            let input = r#"
            {
                "name": "Time Horizon",
                "dataType": "string",
                "sourceColumn": "Name",
                "sortByColumn": "Ordinal"
            }
        "#;

            let column: Column = serde_json::from_str(input).unwrap();

            let output = serde_json::to_string(&column).unwrap();

            assert!(output.contains(r#""sortByColumn":"Ordinal""#))
        }
    }
}

mod partition {
    use super::{Deserialize, Serialize};
    use super::{Expression, Expressive};

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
}

mod measure {
    use super::{Deserialize, Serialize};
    use super::{Expression, Expressive};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct Measure {
        pub name: String,
        expression: Expression,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub format_string: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub display_folder: Option<String>,
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

    #[cfg(test)]
    mod test {
        use super::Expression;
        use super::Measure;

        impl Measure {
            fn new(name: &str, expression: &str) -> Self {
                let expression = Expression::String(expression.to_string());
                Self {
                    name: name.to_string(),
                    expression,
                    format_string: None,
                    display_folder: None,
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

            measures.sort();
            assert_eq!(measures, expected);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CalculationItem {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Expression>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ordinal: Option<i32>,
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

        there_and_back_test(input, CalculationItem::from_value);
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

        there_and_back_test(input, CalculationItem::from_value);
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

        there_and_back_test(input, CalculationItem::from_value);
    }

    #[test]
    fn calculation_item_succeeds_with_name_only() {
        let input = json!(
            {
                "name": "CalculationItem 1"
            }
        );

        there_and_back_test(input, CalculationItem::from_value);
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

        there_and_back_test(input, CalculationGroup::from_value)
    }

    #[test]
    fn can_create_calculation_group_with_empty_calculation_items() {
        let input = json!(
            {
                "calculationItems": []
            }
        );
        there_and_back_test(input, CalculationGroup::from_value);
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

        there_and_back_test(input, Table::from_value);
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

        there_and_back_test(input, Table::from_value);
    }
}
