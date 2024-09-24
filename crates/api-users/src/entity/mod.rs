use sellershut_core::google::protobuf::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub public_key_pem: String,
    pub private_key_pem: Option<String>,
    pub inbox: String,
    pub last_refreshed_at: OffsetDateTime,
    pub followers: Vec<String>,
    pub local: bool,
    pub ap_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TryFrom<sellershut_core::users::User> for User {
    type Error = tonic::Status;

    fn try_from(value: sellershut_core::users::User) -> Result<Self, Self::Error> {
        let created_at = convert_timestamp(value.created_at)?;

        let updated_at = convert_timestamp(value.updated_at)?;
        let last_refreshed_at = convert_timestamp(value.last_refreshed_at)?;

        Ok(Self {
            id: value.id,
            username: value.username,
            ap_id: value.ap_id,
            created_at,
            followers: value.followers,
            last_refreshed_at,
            updated_at,
            local: value.local,
            inbox: value.inbox,
            private_key_pem: value.private_key_pem,
            public_key_pem: value.public_key_pem,
        })
    }
}

impl From<User> for sellershut_core::users::User {
    fn from(value: User) -> Self {
        let created_at = Some(Timestamp::from(value.created_at));
        let updated_at = Some(Timestamp::from(value.updated_at));
        let last_refreshed_at = Some(Timestamp::from(value.last_refreshed_at));

        Self {
            id: value.id,
            username: value.username,
            ap_id: value.ap_id,
            created_at,
            followers: value.followers,
            last_refreshed_at,
            updated_at,
            local: value.local,
            inbox: value.inbox,
            private_key_pem: value.private_key_pem,
            public_key_pem: value.public_key_pem,
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
