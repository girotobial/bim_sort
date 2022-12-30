use super::annotations::Annotation;
use super::expression::ModelExpression;
use super::table::Table;
use super::traits::RecursiveSort;
use super::{datasource::DataSource, relationship::Relationship, roles::Role};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub culture: String,
    pub discourage_implicit_measures: bool,
    pub data_sources: Vec<DataSource>,
    pub tables: Vec<Table>,
    pub relationships: Vec<Relationship>,
    pub roles: Vec<Role>,
    pub expressions: Vec<ModelExpression>,
    pub annotations: Vec<Annotation>,
}

impl RecursiveSort for Model {
    fn recursive_sort(&mut self) {
        self.data_sources.sort();
        self.tables.recursive_sort();
        self.relationships.sort();
    }
}
