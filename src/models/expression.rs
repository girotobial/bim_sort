use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ModelExpression {
    pub name: String,

    pub kind: String,
    expression: Expression,
}

impl Expressive for ModelExpression {
    fn expression(&self) -> Option<String> {
        Some(self.expression.to_string())
    }
}
