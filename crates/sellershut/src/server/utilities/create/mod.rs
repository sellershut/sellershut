use activitypub_federation::config::Data;
use activitypub_federation::{
    fetch::object_id::ObjectId, kinds::activity::CreateType,
    protocol::helpers::deserialize_one_or_many, traits::Activity,
};
use async_trait::async_trait;
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
#[enum_delegate::implement(Activity)]
pub enum CreatableObject {
    CategoryScheme(FederatedCategoryScheme),
}

#[async_trait]
impl<O> Activity for Create<O>
where
    O: Activity + Send + Sync,
{
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = O::DataType;

    #[doc = " Error type returned by handler methods"]
    type Error = O::Error;

    #[doc = " `id` field of the activity"]
    fn id(&self) -> &Url {
        self.object.id()
    }

    #[doc = " `actor` field of activity"]
    fn actor(&self) -> &Url {
        self.object.actor()
    }

    #[doc = " Verifies that the received activity is valid."]
    #[doc = ""]
    #[doc = " This needs to be a separate method, because it might be used for activities"]
    #[doc = " like `Undo/Follow`, which shouldn\'t perform any database write for the inner `Follow`."]
    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // self.object.verify(data).await?;
        Ok(())
    }

    #[doc = " Called when an activity is received."]
    #[doc = ""]
    #[doc = " Should perform validation and possibly write action to the database. In case the activity"]
    #[doc = " has a nested `object` field, must call `object.from_json` handler."]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        self.object.receive(data).await?;
        Ok(())
    }
}
