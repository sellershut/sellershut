pub mod mutation;
pub mod query;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::object::{ImageType, ObjectType, PlaceType},
    traits::Object,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

use crate::{server::error::AppError, state::AppHandle};

use super::user::HutUser;

#[derive(Debug, Clone)]
pub struct HutListing(sellershut_core::listings::Listing);

impl From<HutListing> for Listing {
    fn from(value: HutListing) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    #[serde(rename = "type")]
    kind: ObjectType,
    name: String,
    summary: String,
    location: Location,
    #[serde(rename = "attributedTo")]
    attributed_to: ObjectId<HutUser>,
    id: ObjectId<HutListing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    end_time: Option<OffsetDateTime>,
    published: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "type")]
    kind: PlaceType,
    latitude: f32,
    longitude: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    kind: ImageType,
    name: String,
    url: Url,
}

#[tonic::async_trait]
impl Object for HutListing {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppHandle;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Listing;

    #[doc = " Error type returned by handler methods"]
    type Error = AppError;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
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
    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
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
        data: &Data<Self::DataType>,
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
