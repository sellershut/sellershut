use sellershut_core::user::User;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct DatabaseActor {
    pub id: Uuid,
    pub ap_id: String,
    pub preferred_username: String,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub inbox: String,
    pub outbox: String,
    pub followers: Option<String>,
    pub following: Option<String>,
    pub likes: Option<String>,
    pub icon: Option<String>,
    pub kind: String,
    pub is_local: bool,
    pub last_refreshed_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub public_key: String,
}

impl From<DatabaseActor> for User {
    fn from(value: DatabaseActor) -> Self {
        Self {
            id: value.id,
            ap_id: value.ap_id.into(),
            preferred_username: value.preferred_username,
            name: value.name,
            summary: value.summary,
            inbox: value.inbox.into(),
            outbox: value.outbox.into(),
            following: value.following.map(Into::into),
            followers: value.followers.map(Into::into),
            likes: value.likes.map(Into::into),
            icon: value.icon,
            kind: value.kind,
            is_local: value.is_local,
            created_at: value.created_at,
            updated_at: value.updated_at,
            last_refreshed_at: value.last_refreshed_at,
            public_key: value.public_key,
        }
    }
}
