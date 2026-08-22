use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::Url;

#[derive(Clone, Debug, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CategoryScheme {
    pub id: Uuid,
    pub ap_id: Url,
    pub name: String,
    pub owner_ap_id: Option<Url>,
    pub top_concepts_ap_id: Option<Url>,
    pub is_local: bool,
    pub ap_published_at: Option<OffsetDateTime>,
    pub ap_updated_at: Option<OffsetDateTime>,
    pub last_refreshed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug)]
pub struct CreateCategoryScheme {
    pub id: Uuid,
    pub ap_id: String,
    pub name: String,
    pub owner_ap_id: Option<String>,
    pub is_local: bool,
}

impl CreateCategoryScheme {
    pub fn local(
        ap_id: impl AsRef<str>,
        name: impl AsRef<str>,
        owner_ap_id: impl AsRef<str>,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            ap_id: ap_id.as_ref().to_string(),
            name: name.as_ref().to_string(),
            owner_ap_id: Some(owner_ap_id.as_ref().to_string()),
            is_local: true,
        }
    }
}
