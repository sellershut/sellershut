use activitypub_federation::{
    fetch::object_id::ObjectId, kinds::activity::CreateType,
    protocol::helpers::deserialize_one_or_many,
};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::server::entities::{category::scheme::FederatedCategoryScheme, user::User};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Create<O> {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[schema(value_type = String)]
    pub actor: ObjectId<User>,
    #[serde(
        default,
        deserialize_with = "deserialize_one_or_many",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub to: Vec<Url>,
    #[serde(
        default,
        deserialize_with = "deserialize_one_or_many",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cc: Vec<Url>,
    pub object: O,
    #[serde(rename = "type")]
    #[schema(value_type = String)]
    pub kind: CreateType,
    pub id: Url,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum CreatableObject {
    CategoryScheme(FederatedCategoryScheme),
}
