use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::time::OffsetDateTime};

fn default_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

#[derive(
    SimpleObject, InputObject, FromRow, Debug, Serialize, Deserialize, PartialEq, Eq, Clone,
)]
#[graphql(input_name = "CategoryInput")]
pub struct Category {
    #[graphql(skip_input)]
    pub id: String,
    pub name: String,
    #[graphql(default)]
    pub sub_categories: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<String>,
    #[graphql(default_with = "default_time()")]
    pub created_at: OffsetDateTime,
    #[graphql(default_with = "default_time()")]
    pub updated_at: OffsetDateTime,
}

impl TryFrom<sellershut_core::categories::Category> for Category {
    type Error = async_graphql::Error;

    fn try_from(value: sellershut_core::categories::Category) -> async_graphql::Result<Self> {
        let created = value
            .created_at
            .ok_or_else(|| async_graphql::Error::new("invalid created at"))?;
        let updated = value
            .updated_at
            .ok_or_else(|| async_graphql::Error::new("invalid updated at"))?;
        Ok(Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: created.try_into()?,
            updated_at: updated.try_into()?,
        })
    }
}

impl From<Category> for sellershut_core::categories::Category {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
        }
    }
}
