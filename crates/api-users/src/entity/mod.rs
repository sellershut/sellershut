pub mod auth;

use sellershut_core::google::protobuf::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub followers: Vec<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TryFrom<sellershut_core::users::User> for User {
    type Error = tonic::Status;

    fn try_from(value: sellershut_core::users::User) -> Result<Self, Self::Error> {
        let created_at = convert_timestamp(value.created_at)?;

        let updated_at = convert_timestamp(value.updated_at)?;

        Ok(Self {
            id: value.id,
            username: value.username,
            created_at,
            followers: value.followers,
            updated_at,
        })
    }
}

impl From<User> for sellershut_core::users::User {
    fn from(value: User) -> Self {
        let created_at = Some(Timestamp::from(value.created_at));
        let updated_at = Some(Timestamp::from(value.updated_at));

        Self {
            id: value.id,
            username: value.username,
            created_at,
            followers: value.followers,
            updated_at,
        }
    }
}

fn convert_timestamp(timestamp: Option<Timestamp>) -> Result<OffsetDateTime, tonic::Status> {
    match timestamp.map(OffsetDateTime::try_from) {
        Some(Ok(result)) => Ok(result),
        _ => Err(tonic::Status::invalid_argument(
            "timestamp column is invalid for user",
        )),
    }
}
