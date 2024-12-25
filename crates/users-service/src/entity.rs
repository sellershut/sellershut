use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub followers: Followers,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_refreshed_at: OffsetDateTime,
    pub local: bool,
    pub private_key: Option<String>,
    pub public_key: String,
    pub inbox: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Followers {
    col: HashSet<String>,
}

impl From<Vec<std::string::String>> for Followers {
    fn from(value: Vec<std::string::String>) -> Self {
        Self {
            col: value.into_iter().collect::<HashSet<_>>(),
        }
    }
}

impl From<User> for sellershut_core::users::User {
    fn from(value: User) -> Self {
        Self {
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            followers: value.followers.col.into_iter().collect::<Vec<_>>(),
            id: value.id,
            avatar_url: value.avatar_url,
            email: value.email,
            username: value.username,
            last_refreshed_at: value.last_refreshed_at.into(),
            local: value.local,
            inbox: value.inbox.to_string(),
            private_key: value.private_key,
            public_key: value.public_key,
        }
    }
}

impl TryFrom<sellershut_core::users::User> for User {
    type Error = anyhow::Error;

    fn try_from(value: sellershut_core::users::User) -> anyhow::Result<Self> {
        let created = value.created_at;
        let updated = value.updated_at;
        Ok(Self {
            id: value.id,
            username: value.username,
            avatar_url: value.avatar_url,
            email: value.email,
            followers: Followers {
                col: value.followers.into_iter().collect::<HashSet<_>>(),
            },
            created_at: created.try_into()?,
            updated_at: updated.try_into()?,
            inbox: value.inbox,
            last_refreshed_at: value.last_refreshed_at.try_into()?,
            local: value.local,
            private_key: value.private_key,
            public_key: value.public_key,
        })
    }
}
