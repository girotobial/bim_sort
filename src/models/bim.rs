use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BimFile {
    name: String,
    compatibility_level: u32,
    model: Model,
    id: String,
}
