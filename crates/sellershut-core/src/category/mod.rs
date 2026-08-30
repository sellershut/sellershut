mod id;
pub use id::*;

mod scheme;
pub use scheme::*;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Category {
    pub id: CategoryId,
    pub ap_id: String,
    pub scheme_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<CategoryId>,
    pub is_local: bool,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::rfc3339"))]
    pub created_at: OffsetDateTime,
    #[cfg_attr(feature = "serde", serde(with = "time::serde::rfc3339"))]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug)]
pub struct CreateCategory {
    pub id: CategoryId,
    pub ap_id: String,
    pub scheme_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<CategoryId>,
    pub is_local: bool,
}

impl CreateCategory {
    pub fn local(
        ap_id: impl AsRef<str>,
        scheme_id: Uuid,
        name: impl AsRef<str>,
        parent_id: Option<CategoryId>,
    ) -> Self {
        Self {
            id: CategoryId::new(),
            ap_id: ap_id.as_ref().to_string(),
            scheme_id,
            name: name.as_ref().to_string(),
            description: None,
            image_url: None,
            parent_id,
            is_local: true,
        }
    }
}
