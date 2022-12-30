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

pub use member::Member;
pub use role::Role;

mod role {
    use crate::models::traits::RecursiveSort;

    use super::member::Member;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Role {
        pub name: String,
        pub model_permission: String,
        pub members: Vec<Member>,
    }

    impl RecursiveSort for Role {
        fn recursive_sort(&mut self) {
            self.members.sort();
        }
    }

    impl Ord for Role {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.name.to_lowercase().cmp(&other.name.to_lowercase())
        }
    }

    impl PartialOrd for Role {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
}

mod member {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Member {
        #[serde(rename = "memberName")]
        pub name: String,

        #[serde(rename = "memberId")]
        pub id: String,

        pub identity_provider: String,
    }

    impl Ord for Member {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.name.to_lowercase().cmp(&other.name.to_lowercase())
        }
    }

    impl PartialOrd for Member {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
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
