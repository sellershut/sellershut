use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::server::error::{ApiResult, AppError};

#[derive(SimpleObject, InputObject, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[graphql(input_name = "CategoryInput", name = "Category")]
pub struct GraphQLCategoryType {
    #[graphql(skip_input)]
    pub id: String,
    #[graphql(skip_input)]
    pub ap_id: String,
    pub name: String,
    #[graphql(default)]
    pub sub_categories: Vec<String>,
    #[graphql(skip)]
    pub local: bool,
    pub image_url: Option<String>,
    pub parent_id: Option<String>,
    #[graphql(default_with = "default_time()")]
    pub created_at: OffsetDateTime,
    #[graphql(default_with = "default_time()")]
    pub updated_at: OffsetDateTime,
}

impl TryFrom<sellershut_core::categories::Category> for GraphQLCategoryType {
    type Error = AppError;

    fn try_from(value: sellershut_core::categories::Category) -> ApiResult<Self> {
        Ok(Self {
            id: value.id,
            ap_id: value.ap_id,
            name: value.name,
            local: value.local,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: value
                .created_at
                .ok_or_else(|| anyhow::anyhow!("missing created_at"))?
                .try_into()?,
            updated_at: value
                .updated_at
                .ok_or_else(|| anyhow::anyhow!("missing updated_at"))?
                .try_into()?,
        })
    }
}

impl From<GraphQLCategoryType> for sellershut_core::categories::Category {
    fn from(value: GraphQLCategoryType) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
            ap_id: value.ap_id,
            local: value.local,
        }
    }
}

fn default_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
