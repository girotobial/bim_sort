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

use crate::models::annotations::Annotation;
use crate::models::expression::{Expression, Expressive};
use crate::models::traits::RecursiveSort;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", deny_unknown_fields)]
pub enum Column {
    Calculated(Calculated),
    Sourced(Sourced),
    CalculatedTableColumn(CalculatedTableColumn),
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
            Self::CalculatedTableColumn(c) => c.name(),
        }
    }
    fn data_type(&self) -> String {
        match self {
            Self::Calculated(c) => c.data_type(),
            Self::Sourced(c) => c.data_type(),
            Self::CalculatedTableColumn(c) => c.data_type(),
        }
    }
    fn is_hidden(&self) -> bool {
        match self {
            Self::Calculated(c) => c.is_hidden(),
            Self::Sourced(c) => c.is_hidden(),
            Self::CalculatedTableColumn(c) => c.is_hidden(),
        }
    }
}

impl Expressive for Column {
    fn expression(&self) -> Option<String> {
        match self {
            Self::Calculated(c) => c.expression(),
            _ => None,
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

    #[serde(skip_serializing_if = "DataCategory::is_uncategorized", default)]
    data_category: DataCategory,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_hidden: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
enum DataCategory {
    Uncategorized,
    Address,
    City,
    Continent,
    Country,
    County,
    Image,
    ImageUrl,
    Latitude,
    Longitude,
    Organization,
    Place,
    PostalCode,
    StateOrProvince,
    WebUrl,
}

impl Default for DataCategory {
    fn default() -> Self {
        Self::Uncategorized
    }
}

impl DataCategory {
    fn is_uncategorized(&self) -> bool {
        self.eq(&Self::Uncategorized)
    }
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
pub struct CalculatedTableColumn {
    #[serde(flatten)]
    common: CommonColumn,

    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_inferred: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_data_type_inferred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_string: Option<String>,

    source_column: String,
}

impl Attributes for CalculatedTableColumn {
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
    use crate::models::test::there_and_back_test;
    use crate::models::test::FromValue;

    use super::Column;
    use super::Expressive;
    use serde_json;
    use serde_json::json;

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
    use super::DataCategory;
    use super::Sourced;

    impl Column {
        fn new_calculated(name: &str, data_type: &str, expression: &str) -> Self {
            Self::Calculated(Calculated {
                common: CommonColumn {
                    name: name.to_string(),
                    data_type: data_type.to_string(),
                    is_hidden: None,
                    data_category: DataCategory::default(),
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
                    data_category: DataCategory::default(),
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

    #[test]
    fn columns_have_data_categories() {
        let column = json!(
            {
                "name": "Latitude",
                "dataType": "decimal",
                "sourceColumn": "Latitude",
                "dataCategory": "Latitude"
            }
        );

        there_and_back_test(&column, Column::from_value);
    }

    #[test]
    fn can_parse_all_data_categories() {
        let categories = [
            "Address",
            "City",
            "Continent",
            "Country",
            "County",
            "Image",
            "ImageUrl",
            "Latitude",
            "Longitude",
            "Organization",
            "Place",
            "PostalCode",
            "StateOrProvince",
            "WebUrl",
        ];

        for category in categories.into_iter() {
            let data = json!(
                {
                    "name": "ColumnName",
                    "dataType": "decimal",
                    "sourceColumn": "Column",
                    "dataCategory": category
                }
            );

            there_and_back_test(&data, Column::from_value)
        }
    }

    #[test]
    fn can_parse_calculated_table_columns() {
        let data = json!(
            {
                "type": "calculatedTableColumn",
                "name": "Courses Booked",
                "dataType": "int64",
                "isNameInferred": true,
                "isDataTypeInferred": true,
                "sourceColumn": "[Courses Booked]"
            }
        );

        there_and_back_test(&data, Column::from_value);
    }
}
