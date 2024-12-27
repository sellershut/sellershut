use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{
        link::LinkType,
        object::{ArticleType, ImageType, PlaceType},
    },
    protocol::helpers::deserialize_one_or_many,
    traits::Object,
};
use axum::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

use crate::{hut::Hut, server::error::AppError};

use super::HutUser;

#[derive(Debug, Clone)]
pub struct HutListing(pub sellershut_core::listings::Listing);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    #[serde(rename = "type")]
    kind: ArticleType,
    name: String,    //title
    content: String, //desc
    id: ObjectId<HutListing>,
    pub(crate) attributed_to: ObjectId<HutUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    attachment: Vec<Attachment>,
    location: Location,
    published: OffsetDateTime,
    updated: OffsetDateTime,
    pub(crate) to: Vec<Url>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "type")]
    kind: PlaceType,
    name: String,
    latitude: f32,
    longitude: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "type")]
    kind: ImageType,
    name: String,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    url: Vec<Attachment>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentLink {
    #[serde(rename = "type")]
    kind: LinkType,
    href: Url,
    media_type: String,
}

#[async_trait]
impl Object for HutListing {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = Hut;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Listing;

    #[doc = " Error type returned by handler methods"]
    type Error = AppError;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
    async fn read_from_id(
        _object_id: Url,
        _data: &Data<Self::DataType>,
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
        _json: &Self::Kind,
        _expected_domain: &Url,
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
    async fn from_json(
        _json: Self::Kind,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
