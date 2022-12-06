use serde::{Deserialize, Serialize};

pub use self::column::{Column, ColumnAttributes};
pub use self::partition::{Partition, Source};
use super::annotations::Annotation;
use super::expression::{Expression, Expressive};

use self::measure::Measure;
use super::skip_if::{false_, is_false, is_none};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub name: String,

    #[serde(default = "false_", skip_serializing_if = "is_false")]
    pub is_hidden: bool,
    pub columns: Vec<column::Column>,
    pub partitions: Vec<Partition>,
    pub measures: Option<Vec<Measure>>,
}

mod column {
    use super::{is_none, Annotation};
    use super::{Expression, Expressive};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(untagged, rename_all = "camelCase")]
    pub enum Column {
        Calculated(Calculated),
        Sourced(Sourced),
    }

    impl ColumnAttributes for Column {
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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    struct CommonColumn {
        name: String,
        data_type: String,

        #[serde(skip_serializing_if = "is_none")]
        is_hidden: Option<bool>,
    }

    pub trait ColumnAttributes {
        fn name(&self) -> String;
        fn data_type(&self) -> String;
        fn is_hidden(&self) -> bool;
    }

    impl ColumnAttributes for CommonColumn {
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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Calculated {
        #[serde(flatten)]
        common: CommonColumn,

        #[serde(rename = "type")]
        pub type_: String,
        expression: Expression,

        #[serde(skip_serializing_if = "is_none")]
        pub is_data_type_inferred: Option<bool>,

        #[serde(skip_serializing_if = "is_none")]
        pub format_string: Option<String>,
    }

    impl Expressive for Calculated {
        fn expression(&self) -> Option<String> {
            Some(self.expression.as_string())
        }
    }

    impl ColumnAttributes for Calculated {
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

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Sourced {
        #[serde(flatten)]
        common: CommonColumn,
        pub source_column: String,

        #[serde(skip_serializing_if = "is_none")]
        pub description: Option<String>,

        #[serde(skip_serializing_if = "is_none")]
        pub format_string: Option<String>,

        #[serde(skip_serializing_if = "is_none")]
        pub is_hidden: Option<bool>,

        #[serde(skip_serializing_if = "is_none")]
        pub annotations: Option<Vec<Annotation>>,
    }

    impl ColumnAttributes for Sourced {
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

            let column: Column = serde_json::from_str(column_content).unwrap();
            assert_eq!(column.expression().unwrap(), expected_expression);
        }
    }
}

mod partition {
    use super::is_none;
    use super::{Deserialize, Serialize};
    use super::{Expression, Expressive};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Partition {
        pub name: String,
        #[serde(skip_serializing_if = "is_none")]
        pub data_view: Option<String>,

        pub source: Source,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Source {
        #[serde(rename = "type")]
        pub type_: String,

        #[serde(skip_serializing_if = "is_none")]
        expression: Option<Expression>,
    }

    impl Expressive for Source {
        fn expression(&self) -> Option<String> {
            self.expression.as_ref().map(Expression::as_string)
        }
    }
}

mod measure {
    use super::{Deserialize, Serialize};
    use super::{Expression, Expressive};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Measure {
        pub name: String,
        expression: Expression,
        pub format_string: Option<String>,
        pub display_folder: Option<String>,
    }

    impl Expressive for Measure {
        fn expression(&self) -> Option<String> {
            Some(self.expression.as_string())
        }
    }
}
