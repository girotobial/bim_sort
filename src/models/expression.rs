use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Expression {
    Vec(Vec<String>),
    String(String),
}

impl Expression {
    pub fn as_string(&self) -> String {
        match self {
            Expression::Vec(v) => v.join("\n"),
            Expression::String(s) => s.clone(),
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.as_string() == other.as_string()
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
        Some(self.expression.as_string())
    }
}
