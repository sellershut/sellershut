use sqlx::prelude::Type;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: Option<String>,
    pub inbox: String,
    pub public_key: String,
    pub kind: ActorType,
    pub private_key: Option<String>,
    pub created_at: OffsetDateTime,
    pub last_refreshed_at: OffsetDateTime,
    pub is_local: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_kind")]
#[sqlx(rename_all = "lowercase")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActorType {
    Person,
    Service,
    Organization,
    Group,
    Application,
}
