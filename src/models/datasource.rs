use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DataSource {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,

    #[serde(rename = "connectionDetails")]
    pub connection_details: ConnectionDetails,
    pub credential: CredentialType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "protocol")]
pub enum ConnectionDetails {
    #[serde(rename = "document-db")]
    DocumentDb { address: Address },

    #[serde(rename = "tds")]
    Tds {
        address: Address,
        authentication: Option<String>,
        query: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Address {
    DocumentDb {
        url: String,
        database: Option<String>,
        collection: Option<String>,
    },
    Tds {
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
pub struct CredentialCommon {
    kind: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "AuthenticationKind")]
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
    fn kind(&self) -> String {
        match self {
            Self::Key { common } | Self::UsernamePassword { common, .. } => common.kind.clone(),
        }
    }

    fn path(&self) -> String {
        match self {
            Self::Key { common } | Self::UsernamePassword { common, .. } => common.path.clone(),
        }
    }

    fn authentication(&self) -> Authentication {
        match self {
            Self::Key { .. } => Authentication::Key,
            Self::UsernamePassword { .. } => Authentication::UsernamePassword,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

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
}
