use super::datasource::DataSource;
use super::table::Table;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    culture: String,
    discourage_implicit_measures: bool,
    data_sources: Vec<DataSource>,
    tables: Vec<Table>,
}
