pub mod mutation;
pub mod query;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::object::{ImageType, ObjectType, PlaceType},
    traits::Object,
};
use anyhow::anyhow;
use sellershut_core::listings::{GetListingByApIdRequest, UpsertistingRequest};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tonic::IntoRequest;
use tracing::{Instrument, debug, info_span};
use url::Url;

use crate::{server::error::AppError, state::AppHandle};

use super::{
    category::{CategoryItem, HutCategory},
    user::HutUser,
};

#[derive(Debug, Clone)]
pub struct HutListing(sellershut_core::listings::Listing);

impl TryFrom<HutListing> for Listing {
    type Error = AppError;

    fn try_from(value: HutListing) -> Result<Self, Self::Error> {
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
    updated: OffsetDateTime,
    attachment: Vec<Media>,
    image: Media,
    // category?? more tags?
    tag: CategoryItem,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(rename = "type")]
    kind: ImageType,
    url: Url,
}

impl TryFrom<Listing> for HutListing {
    type Error = AppError;

    fn try_from(value: Listing) -> Result<Self, Self::Error> {
        todo!()
    }
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
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut client = data.query_listings_client.clone();
        let query_by_ap_id = GetListingByApIdRequest {
            ap_id: object_id.to_string(),
        };

        debug!(id = ?object_id, "getting listing");

        let resp = client
            .listings_by_ap_id(query_by_ap_id.into_request())
            .instrument(info_span!("grpc.listing.get"))
            .await?
            .into_inner()
            .listing;

        if let Some(resp) = resp {
            debug!(id = ?object_id, "listing found {resp:?}");
            let listing = HutListing(resp);
            Ok(Some(listing))
        } else {
            debug!(id = ?object_id, "listing not found");
            Ok(None)
        }
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Self::Kind::try_from(self)
    }

    #[doc = " Verifies that the received object is valid."]
    #[doc = ""]
    #[doc = " You should check here that the domain of id matches `expected_domain`. Additionally you"]
    #[doc = " should perform any application specific checks."]
    #[doc = ""]
    #[doc = " It is necessary to use a separate method for this, because it might be used for activities"]
    #[doc = " like `Delete/Note`, which shouldn\'t perform any database write for the inner `Note`."]
    async fn verify(
        _json: &Self::Kind,
        _expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    #[doc = " Convert object from ActivityPub type to database type."]
    #[doc = ""]
    #[doc = " Called when an object is received from HTTP fetch or as part of an activity. This method"]
    #[doc = " should write the received object to database. Note that there is no distinction between"]
    #[doc = " create and update, so an `upsert` operation should be used."]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let id = json.id;

        debug!(id = ?id, "upserting listing");

        let request = UpsertistingRequest {
            listing: Some(sellershut_core::listings::Listing {
                user_ap_id: json.attributed_to.into_inner().to_string(),
                title: json.name,
                description: json.summary,
                expires_at: json.end_time.map(Into::into),
                created_at: Some(json.published.into()),
                ap_id: id.clone().into_inner().to_string(),
                quantity: todo!(),
                status: todo!(),
                price: todo!(),
                liked_by: todo!(),
                category_ap_id: todo!(),
                location: todo!(),
                city: todo!(),
                region: todo!(),
                country: todo!(),
                ..Default::default()
            }),
        }
        .into_request();

        let mut client = data.mutate_listings_client.clone();
        let resp = client
            .upsert_listing(request)
            .instrument(info_span!("grpc.listing.upsert"))
            .await?
            .into_inner()
            .listing
            .ok_or_else(|| anyhow!("listing not returned from upsert"))?;
        debug!(id = ?id, "listing upserted");

        Ok(HutListing(resp))
    }
}
