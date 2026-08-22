use time::OffsetDateTime;
use uuid::Uuid;

use crate::{custom_url::Url};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct User {
    pub id: Uuid,
    pub ap_id: Url,

    pub preferred_username: String,
    pub name: Option<String>,
    pub summary: Option<String>,

    pub inbox: Url,
    pub outbox: Url,

    pub following: Option<Url>,
    pub followers: Option<Url>,

    pub likes: Option<Url>,
    pub icon: Option<String>,
    pub kind: String,

    pub is_local: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_refreshed_at: OffsetDateTime,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ActorKey {
    id: Uuid,
    actor_id: Url,
    public_key_pem: String
}
//
// #[cfg(feature = "serde")]
// mod s {
//
//     use serde::Serializer;
//     pub(super) fn serialize_redacted_secret<S>(
//         s: &Option<crate::RedactedSecret>,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match s {
//             Some(v) => serializer.serialize_str(&v.expose()),
//             None => serializer.serialize_none(),
//         }
//     }
// }
