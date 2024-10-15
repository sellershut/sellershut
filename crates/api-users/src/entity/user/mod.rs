use std::collections::HashSet;

use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(SimpleObject, InputObject, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub followers: Followers,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(SimpleObject, InputObject, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
struct Followers {
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
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
            email: value.email,
            followers: value.followers.col.into_iter().collect::<Vec<_>>(),
            id: value.id,
            avatar_url: value.avatar_url,
            username: value.username,
        }
    }
}

impl TryFrom<sellershut_core::users::User> for User {
    type Error = async_graphql::Error;

    fn try_from(value: sellershut_core::users::User) -> async_graphql::Result<Self> {
        let created = value
            .created_at
            .ok_or_else(|| async_graphql::Error::new("invalid created at"))?;
        let updated = value
            .updated_at
            .ok_or_else(|| async_graphql::Error::new("invalid updated at"))?;
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
        })
    }
}
