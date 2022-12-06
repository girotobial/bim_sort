pub use member::Member;
pub use role::Role;

mod role {
    use super::member::Member;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Role {
        pub name: String,
        pub model_permission: String,
        pub members: Vec<Member>,
    }
}

mod member {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Member {
        #[serde(rename = "memberName")]
        pub name: String,

        #[serde(rename = "memberId")]
        pub id: String,

        pub identity_provider: String,
    }

    #[cfg(test)]
    mod test {
        use super::Member;
        use serde_json;

        #[test]
        fn can_create_member_from_string() {
            let input = r#"
        {
            "memberName": "user.name@username.com",
            "memberId": "user.name2@username.com",
            "identityProvider": "AzureAD"
        }
        "#;

            let member: Member = serde_json::from_str(input).unwrap();

            assert_eq!(member.id, "user.name2@username.com".to_string());
            assert_eq!(member.name, "user.name@username.com".to_string());
            assert_eq!(member.identity_provider, "AzureAD".to_string());
        }
    }
}
