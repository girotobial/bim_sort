use serde::{Deserialize, Serialize};

use super::prelude::Model;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BimFile {
    pub name: String,
    pub compatibility_level: u32,
    pub model: Model,
    pub id: String,
}
