use super::table::Table;
use super::{datasource::DataSource, relationship::Relationship};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub culture: String,
    pub discourage_implicit_measures: bool,
    pub data_sources: Vec<DataSource>,
    pub tables: Vec<Table>,
    pub relationships: Vec<Relationship>,
}
