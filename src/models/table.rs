use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    name: String,

    #[serde(default = "false_", skip_serializing_if = "is_false")]
    is_hidden: bool,
    columns: Vec<column::ColumnType>,
    // partitions: Vec<Partition>,
}

mod column {
    use super::expression::{Expression, Expressive};
    use super::{is_none, Annotation};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged, rename_all = "camelCase")]
    pub enum ColumnType {
        Calculated(Calculated),
        Sourced(Sourced),
    }

    impl Expressive for ColumnType {
        fn expression(&self) -> Option<String> {
            match self {
                ColumnType::Calculated(c) => c.expression(),
                _ => None,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ColumnCommon {
        pub name: String,
        pub data_type: String,

        #[serde(skip_serializing_if = "is_none")]
        pub is_hidden: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Calculated {
        #[serde(flatten)]
        common: ColumnCommon,

        #[serde(rename = "type")]
        pub type_: String,
        expression: Expression,

        #[serde(skip_serializing_if = "is_none")]
        is_data_type_inferred: Option<bool>,

        #[serde(skip_serializing_if = "is_none")]
        format_string: Option<String>,
    }

    impl Expressive for Calculated {
        fn expression(&self) -> Option<String> {
            Some(self.expression.as_string())
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Sourced {
        #[serde(flatten)]
        common: ColumnCommon,
        source_column: String,

        #[serde(skip_serializing_if = "is_none")]
        description: Option<String>,

        #[serde(skip_serializing_if = "is_none")]
        format_string: Option<String>,

        #[serde(skip_serializing_if = "is_none")]
        is_hidden: Option<bool>,

        #[serde(skip_serializing_if = "is_none")]
        annotations: Option<Vec<Annotation>>,
    }

    #[cfg(test)]
    mod tests {
        use crate::models::table::expression::Expressive;

        use super::ColumnType;
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

            let column: ColumnType = serde_json::from_str(&column_content).unwrap();
            assert_eq!(column.expression().unwrap(), expected_expression);
        }
    }
}

mod expression {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(untagged)]
    pub enum Expression {
        Vec(Vec<String>),
        String(String),
    }

    pub trait Expressive {
        fn expression(&self) -> Option<String>;
    }

    impl Expression {
        pub fn as_string(&self) -> String {
            match self {
                Expression::Vec(contents) => contents.join("\n"),
                Expression::String(s) => s.clone(),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Annotation {
    name: String,
    value: String,
}

fn false_() -> bool {
    false
}

fn is_false(x: &bool) -> bool {
    !x
}

fn is_none<T>(option: &Option<T>) -> bool {
    option.is_none()
}

mod partition {
    use super::expression::Expression;
    use super::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Partition {
        pub name: String,
        pub data_view: String,
        pub source: Source,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Source {
        #[serde(rename = "type")]
        pub type_: String,
        expression: Expression,
    }
}
