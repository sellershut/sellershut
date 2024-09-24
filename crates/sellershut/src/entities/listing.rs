use crate::state::AppState;
use activitypub_federation::{
    config::Data, fetch::object_id::ObjectId, kinds::object::ObjectType,
    protocol::helpers::deserialize_one_or_many, traits::Object,
};
use sellershut_core::listings::Listing as DbListing;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use url::Url;

use super::user::LocalUser;

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalListing(DbListing);

impl From<DbListing> for LocalListing {
    fn from(value: DbListing) -> Self {
        Self(value)
    }
}

impl LocalListing {
    pub fn get(&self) -> &DbListing {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Listing {
    #[serde(rename = "type")]
    kind: ObjectType,
    id: ObjectId<LocalListing>,
    pub attributed_to: ObjectId<LocalUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    content: LocalListing,
}

#[tonic::async_trait]
impl Object for LocalListing {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppState;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Listing;

    #[doc = " Error type returned by handler methods"]
    type Error = tonic::Status;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
    #[instrument(skip(data))]
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        todo!()
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    #[must_use]
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        todo!()
    }

    #[doc = " Verifies that the received object is valid."]
    #[doc = ""]
    #[doc = " You should check here that the domain of id matches `expected_domain`. Additionally you"]
    #[doc = " should perform any application specific checks."]
    #[doc = ""]
    #[doc = " It is necessary to use a separate method for this, because it might be used for activities"]
    #[doc = " like `Delete/Note`, which shouldn\'t perform any database write for the inner `Note`."]
    #[must_use]
    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    #[doc = " Convert object from ActivityPub type to database type."]
    #[doc = ""]
    #[doc = " Called when an object is received from HTTP fetch or as part of an activity. This method"]
    #[doc = " should write the received object to database. Note that there is no distinction between"]
    #[doc = " create and update, so an `upsert` operation should be used."]
    #[must_use]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        todo!()
    }
}
