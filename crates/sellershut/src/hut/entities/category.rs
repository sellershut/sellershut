use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{collection::CollectionPageType, object::ImageType},
    protocol::verification::verify_domains_match,
    traits::Object,
};
use axum::async_trait;
use opentelemetry_semantic_conventions::trace;
use sellershut_core::categories::{GetCategoryRequest, UpsertCategoryRequest};
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use tracing::{Instrument, debug};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use url::Url;

use crate::{hut::Hut, server::error::AppError};

#[derive(Debug, Clone)]
pub struct HutCategory(pub sellershut_core::categories::Category);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "type")]
    kind: CollectionPageType,
    name: String,
    id: ObjectId<HutCategory>,
    total_items: usize,
    part_of: Option<ObjectId<HutCategory>>,
    items: Vec<Category>,
    image: Option<CategoryImage>,
    next: Option<ObjectId<HutCategory>>,
    prev: Option<ObjectId<HutCategory>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryImage {
    #[serde(rename = "type")]
    kind: ImageType,
    name: String,
    url: Url,
}

impl From<Category> for sellershut_core::categories::Category {
    fn from(_value: Category) -> Self {
        todo!()
    }
}

impl TryFrom<HutCategory> for Category {
    type Error = AppError;

    fn try_from(_value: HutCategory) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl Object for HutCategory {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = Hut;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Category;

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
        let mut client = data.query_categories_client.clone();
        let req = GetCategoryRequest {
            id: object_id.to_string(),
        };
        debug!(id = ?object_id, "getting category");

        let resp = client
            .category_by_id(req)
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "QueryCategoriesClient");
                span.set_attribute(trace::RPC_METHOD, "CategoryById");
                span
            })
            .await?
            .into_inner()
            .category;

        if let Some(resp) = resp {
            debug!("category found {resp:?}");
            Ok(Some(HutCategory(resp)))
        } else {
            debug!("category not found");
            Ok(None)
        }
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
        let category = sellershut_core::categories::Category::from(json);

        let mut client = data.mutate_categories_client.clone();

        let upsert_req = UpsertCategoryRequest {
            category: Some(category),
        }
        .into_request();

        let resp = client
            .upsert(upsert_req)
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "MutateCategoriesClient");
                span.set_attribute(trace::RPC_METHOD, "Upsert");
                span
            })
            .await?
            .into_inner();

        Ok(HutCategory(resp))
    }
}
