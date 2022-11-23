use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSource {
    #[serde(rename = "type")]
    type_: String,
    name: String,

    #[serde(rename = "connectionDetails")]
    connection_details: ConnectionDetails,
    credential: Credential,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "protocol")]
enum ConnectionDetails {
    #[serde(rename = "document-db")]
    DocumentDb { address: Address },

    #[serde(rename = "tds")]
    Tds {
        address: Address,
        authentication: Option<String>,
        query: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Address {
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

#[derive(Serialize, Deserialize, Debug)]
struct CredentialCommon {
    kind: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "AuthenticationKind")]
enum Credential {
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
