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
            Expression::Vec(v) => v.join("\n"),
            Expression::String(s) => s.clone(),
        }
    }
}
