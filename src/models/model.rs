/*
Bim Sort, sorts bim files for better compatibility with git
Copyright (C) 2022  Alexander Robinson

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use super::annotations::Annotation;
use super::expression::ModelExpression;
use super::skip_if::{is_false, true_};
use super::table::Table;
use super::traits::RecursiveSort;
use super::{datasource::DataSource, relationship::Relationship, roles::Role};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub culture: String,

    #[serde(default = "true_", skip_serializing_if = "is_false")]
    pub discourage_implicit_measures: bool,

    pub data_sources: Vec<DataSource>,
    pub tables: Vec<Table>,
    pub relationships: Vec<Relationship>,
    pub roles: Option<Vec<Role>>,
    pub expressions: Vec<ModelExpression>,
    pub annotations: Vec<Annotation>,
}

impl RecursiveSort for Model {
    fn recursive_sort(&mut self) {
        self.data_sources.sort();
        self.tables.recursive_sort();
        self.relationships.sort();
        {
            if let Some(roles) = &mut self.roles {
                roles.recursive_sort();
            }
        }
        self.expressions.sort();
        self.annotations.sort();
    }
}
