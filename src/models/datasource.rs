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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DataSource {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,

    #[serde(rename = "connectionDetails")]
    pub connection_details: ConnectionDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DataSourceOption>,
    pub credential: CredentialType,
}

impl PartialOrd for DataSource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DataSource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "protocol", deny_unknown_fields)]
pub enum ConnectionDetails {
    #[serde(rename = "document-db")]
    DocumentDb { address: Address },

    #[serde(rename = "tds")]
    Tds(SqlConnection),

    #[serde(rename = "postgresql")]
    PostgresSql(SqlConnection),

    #[serde(rename = "mysql")]
    MySql(SqlConnection),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SqlConnection {
    address: Address,
    authentication: Option<String>,
    query: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum Address {
    DocumentDb {
        url: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        database: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        collection: Option<String>,
    },
    SqlDatabase {
        server: String,
        database: String,
    },
}

pub trait Credential {
    fn kind(&self) -> String;
    fn path(&self) -> String;
    fn authentication(&self) -> Authentication;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CredentialCommon {
    kind: String,
    path: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "PrivacySetting")]
    privacy_setting: Option<PrivacySetting>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PrivacySetting {
    None,
    Public,
    Organizational,
    Private,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "AuthenticationKind", deny_unknown_fields)]
pub enum CredentialType {
    Key {
        #[serde(flatten)]
        common: CredentialCommon,
    },
    UsernamePassword {
        #[serde(flatten)]
        common: CredentialCommon,
        #[serde(rename = "Username")]
        username: String,

        #[serde(rename = "EncryptConnection")]
        encrypt_connection: bool,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub enum Authentication {
    Key,
    UsernamePassword,
}

impl Credential for CredentialType {
    #[must_use]
    fn kind(&self) -> String {
        match self {
            Self::Key { common } | Self::UsernamePassword { common, .. } => common.kind.clone(),
        }
    }

    #[must_use]
    fn path(&self) -> String {
        match self {
            Self::Key { common } | Self::UsernamePassword { common, .. } => common.path.clone(),
        }
    }

    #[must_use]
    fn authentication(&self) -> Authentication {
        match self {
            Self::Key { .. } => Authentication::Key,
            Self::UsernamePassword { .. } => Authentication::UsernamePassword,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DataSourceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    return_single_database: Option<bool>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use serde_json::json;

    use crate::models::test::{there_and_back_test, FromValue};

    #[test]
    fn test_correctly_deserialize_key_credential() {
        let input = r#"
            {
                "AuthenticationKind": "Key",
                "kind": "DocumentDB",
                "path": "https://google.com"
            }
        "#;

        let data: CredentialType = serde_json::from_str(input).unwrap();

        assert_eq!(data.kind(), "DocumentDB".to_string());
        assert_eq!(data.path(), "https://google.com".to_string());
        assert_eq!(data.authentication(), Authentication::Key);
    }

    #[test]
    fn test_correctly_deserialize_key_username_password_credential() {
        let input = r#"
        {
            "AuthenticationKind": "UsernamePassword",
            "kind": "DocumentDB",
            "path": "https://google.com",
            "Username": "MyLogin",
            "EncryptConnection": true
        }
    "#;

        let data: CredentialType = serde_json::from_str(input).unwrap();

        assert_eq!(data.kind(), "DocumentDB".to_string());
        assert_eq!(data.path(), "https://google.com".to_string());
        assert_eq!(data.authentication(), Authentication::UsernamePassword);
    }

    #[test]
    fn test_can_sort_vec_of_datasources() {
        let mut datasources = vec![
            DataSource {
                type_: "structured".to_string(),
                name: "Zero Datasource".to_string(),
                connection_details: ConnectionDetails::DocumentDb {
                    address: Address::DocumentDb {
                        url: "http://google.com".to_string(),
                        database: Some("TheDB".to_string()),
                        collection: Some("Default".to_string()),
                    },
                },
                options: None,
                credential: CredentialType::Key {
                    common: CredentialCommon {
                        kind: "DocumentDb".to_string(),
                        path: "http://google.com".to_string(),
                        privacy_setting: None,
                    },
                },
            },
            DataSource {
                type_: "structured".to_string(),
                name: "A Datasource".to_string(),
                connection_details: ConnectionDetails::DocumentDb {
                    address: Address::DocumentDb {
                        url: "http://google.com".to_string(),
                        database: Some("TheDB".to_string()),
                        collection: Some("Default".to_string()),
                    },
                },
                options: None,
                credential: CredentialType::Key {
                    common: CredentialCommon {
                        kind: "DocumentDb".to_string(),
                        path: "http://google.com".to_string(),
                        privacy_setting: None,
                    },
                },
            },
        ];

        let expected = vec![
            DataSource {
                type_: "structured".to_string(),
                name: "Zero Datasource".to_string(),
                connection_details: ConnectionDetails::DocumentDb {
                    address: Address::DocumentDb {
                        url: "http://google.com".to_string(),
                        database: Some("TheDB".to_string()),
                        collection: Some("Default".to_string()),
                    },
                },
                options: None,
                credential: CredentialType::Key {
                    common: CredentialCommon {
                        kind: "DocumentDb".to_string(),
                        path: "http://google.com".to_string(),
                        privacy_setting: None,
                    },
                },
            },
            DataSource {
                type_: "structured".to_string(),
                name: "A Datasource".to_string(),
                connection_details: ConnectionDetails::DocumentDb {
                    address: Address::DocumentDb {
                        url: "http://google.com".to_string(),
                        database: Some("TheDB".to_string()),
                        collection: Some("Default".to_string()),
                    },
                },
                options: None,
                credential: CredentialType::Key {
                    common: CredentialCommon {
                        kind: "DocumentDb".to_string(),
                        path: "http://google.com".to_string(),
                        privacy_setting: None,
                    },
                },
            },
        ];

        datasources.sort();
        datasources.reverse();
        assert_eq!(expected, datasources);
    }

    #[test]
    fn readwrite_postgressql_connection_details() {
        let data = json!(
            {
                "protocol": "postgresql",
                "address": {
                    "server": "localhost:5432",
                    "database": "flight_db"
                },
                "authentication": null,
                "query": null
            }
        );

        there_and_back_test(&data, ConnectionDetails::from_value);
    }

    #[test]
    fn readwrite_postgressql_datasource() {
        let data = json!(
            {
                "type": "structured",
                "name": "PostgreSQL/localhost:5432;flight_db",
                "connectionDetails": {
                    "protocol": "postgresql",
                    "address": {
                        "server": "localhost:5432",
                        "database": "flight_db"
                    },
                    "authentication": null,
                    "query": null
                },
                "credential": {
                    "AuthenticationKind": "UsernamePassword",
                    "kind": "PostgreSQL",
                    "path": "localhost:5432;flight_db",
                    "Username": "username",
                    "EncryptConnection": false
                }
            }
        );

        there_and_back_test(&data, DataSource::from_value);
    }

    #[test]
    fn readwrite_mysql_connection_details() {
        let data = json!(
            {
                "protocol": "mysql",
                "address": {
                    "server": "db.mysql.database.com",
                    "database": "MySQLDB"
                },
                "authentication": null,
                "query": null
            }
        );

        there_and_back_test(&data, ConnectionDetails::from_value);
    }

    #[test]
    fn can_process_mysql_datasource() {
        let data = json!(
            {
                "type": "structured",
                "name": "DbName",
                "connectionDetails": {
                    "protocol": "mysql",
                    "address": {
                        "server": "db.mysql.database.com",
                        "database": "MySQLDB"
                    },
                    "authentication": null,
                    "query": null
                },
                "options": {
                    "returnSingleDatabase": true
                },
                "credential": {
                    "AuthenticationKind": "UsernamePassword",
                    "kind": "MySql",
                    "path": "db.mysql.database.com;MySQLDB",
                    "Username": "email@emails.com",
                    "EncryptConnection": true,
                    "PrivacySetting": "Organizational"
                }
            }
        );

        there_and_back_test(&data, DataSource::from_value);
    }

    #[test]
    #[should_panic]
    fn credentials_reject_unknown_fields() {
        let input = json!(
            {
                "kind": "A Credential",
                "path": "https://google.com",
                "unknown_field": "Some data"
            }
        );

        CredentialType::from_value(&input);
    }

    #[test]
    fn allow_empty_datasource_options() {
        let datasource = json!(
            {
                "type": "structured",
                "name": "SQL/Aserver",
                "connectionDetails": {
                    "protocol": "tds",
                    "address": {
                        "server": "<serveraddress>",
                        "database": "A database"
                    },
                    "authentication": null,
                    "query": null
                },
                "options": {},
                "credential": {
                    "AuthenticationKind": "UsernamePassword",
                    "kind": "SQL",
                    "path": "<url>",
                    "Username": "username",
                    "EncryptConnection": true
                }
            }
        );

        there_and_back_test(&datasource, DataSource::from_value);
    }
}
