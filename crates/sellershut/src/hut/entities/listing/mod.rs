use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{
        link::LinkType,
        object::{ArticleType, ImageType, PlaceType},
    },
    protocol::{helpers::deserialize_one_or_many, verification::verify_domains_match},
    traits::Object,
};
use axum::async_trait;
use opentelemetry_semantic_conventions::trace;
use sellershut_core::listings::{CreateListingRequest, QueryListingByIdRequest};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tonic::IntoRequest;
use tracing::{Instrument, debug};
use tracing_opentelemetry::OpenTelemetrySpanExt;
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
    // TODO: quantity
    pub(crate) to: Vec<Url>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "type")]
    kind: PlaceType,
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "type")]
    kind: ImageType,
    name: String,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    url: Vec<AttachmentLink>,
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
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut client = data.query_listings_client.clone();
        let query_listing = QueryListingByIdRequest {
            ap_id: object_id.to_string(),
        }
        .into_request();

        Ok(client
            .listings_by_id(query_listing)
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "QueryListingsClient");
                span.set_attribute(trace::RPC_METHOD, "ListingsById");
                span
            })
            .await?
            .into_inner()
            .listing
            .map(|listing| HutListing(listing)))
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    #[must_use]
    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let id = Url::parse(&self.0.ap_id)?;
        let attributed_to = Url::parse(&self.0.user_ap_id)?;

        let attributed_to: ObjectId<HutUser> = attributed_to.into();

        let creator = attributed_to.dereference_local(data).await?;

        Ok(Self::Kind {
            kind: Default::default(),
            id: id.into(),
            attributed_to: attributed_to,
            content: self.0.description.to_string(),
            name: self.0.title.to_string(),
            to: vec![
                activitypub_federation::kinds::public(),
                creator.followers_url()?,
            ],
            location: Location {
                kind: Default::default(),
                latitude: self.0.location.latitude,
                longitude: self.0.location.longitude,
                name: Default::default(),
            },
            updated: self.0.updated_at.try_into()?,
            published: self.0.created_at.try_into()?,
            attachment: self
                .0
                .attachments
                .iter()
                .map(|attachment| {
                    // Propagate error if URL parsing fails
                    Url::parse(attachment).map(|url| Attachment {
                        kind: Default::default(),
                        name: Default::default(),
                        url: vec![AttachmentLink {
                            href: url,
                            kind: Default::default(),
                            media_type: Default::default(),
                        }],
                    })
                })
                .collect::<Result<Vec<_>, url::ParseError>>()?,
        })
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
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    #[doc = " Convert object from ActivityPub type to database type."]
    #[doc = ""]
    #[doc = " Called when an object is received from HTTP fetch or as part of an activity. This method"]
    #[doc = " should write the received object to database. Note that there is no distinction between"]
    #[doc = " create and update, so an `upsert` operation should be used."]
    #[must_use]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let id = json.id;
        debug!(id = ?id, "upserting listing");

        let request = CreateListingRequest {
            listing: sellershut_core::listings::Listing {
                user_ap_id: json.attributed_to.to_string(),
                ap_id: id.clone().into_inner().to_string(),
                attachments: json
                    .attachment
                    .iter()
                    .map(|attachment| {
                        let url: Vec<_> = attachment
                            .url
                            .iter()
                            .map(|url| url.href.to_string())
                            .collect();
                        url
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
                location: sellershut_core::google::r#type::LatLng {
                    latitude: json.location.latitude,
                    longitude: json.location.longitude,
                },
                title: json.name,
                description: json.content,
                created_at: json.published.try_into()?,
                updated_at: json.updated.try_into()?,
                ..Default::default()
            },
        }
        .into_request();

        let mut client = data.mutate_listings_client.clone();
        let resp = client
            .create_listing(request)
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "MutateListingsClient");
                span.set_attribute(trace::RPC_METHOD, "Upsert");
                span
            })
            .await?
            .into_inner()
            .listing;
        debug!(id = ?id, "listing upserted");

        Ok(HutListing(resp))
    }
}
