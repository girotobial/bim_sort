use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq)]
#[serde(untagged)]
pub enum Expression {
    Vec(Vec<String>),
    String(String),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Vec(v) => v.join("\n"),
            Self::String(s) => s.clone(),
        };
        write!(f, "{}", output)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

pub trait Expressive {
    fn expression(&self) -> Option<String>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ModelExpression {
    pub name: String,

    pub kind: String,
    expression: Expression,
}

impl Ord for ModelExpression {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for ModelExpression {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Expressive for ModelExpression {
    #[must_use]
    fn expression(&self) -> Option<String> {
        Some(self.expression.to_string())
    }
}
