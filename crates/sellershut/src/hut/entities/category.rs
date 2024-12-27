use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{collection::CollectionType, object::ImageType},
    protocol::verification::verify_domains_match,
    traits::Object,
};
use axum::async_trait;
use opentelemetry_semantic_conventions::trace;
use sellershut_core::categories::{
    GetAllSubCategoriesRequest, GetCategoryRequest, UpsertCategoryRequest,
};
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use tracing::{Instrument, debug};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use url::Url;

use crate::{hut::Hut, server::error::AppError};

#[derive(Debug, Clone)]
pub struct HutCategory {
    pub id: ObjectId<HutCategory>,
    pub name: String,
    pub sub_categories: Vec<HutCategory>,
    pub image: Option<Url>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "type")]
    kind: CollectionType,
    name: String,
    id: ObjectId<HutCategory>,
    total_items: usize,
    items: Vec<Category>,
    image: Option<CategoryImage>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryImage {
    #[serde(rename = "type")]
    kind: ImageType,
    name: String,
    url: Url,
}

impl From<Category> for sellershut_core::categories::Category {
    fn from(value: Category) -> Self {
        Self {
            name: value.name,
            ap_id: value.id.into_inner().to_string(),
            sub_categories: value.items.into_iter().map(From::from).collect(),
            image_url: value.image.map(|value| value.url.to_string()),
            ..Default::default()
        }
    }
}

impl From<HutCategory> for Category {
    fn from(value: HutCategory) -> Self {
        Self {
            kind: Default::default(),
            name: value.name.clone(),
            id: value.id.into_inner().into(),
            total_items: value.sub_categories.len(),
            items: value.sub_categories.into_iter().map(From::from).collect(),
            image: value.image.map(|category| CategoryImage {
                kind: Default::default(),
                name: format!("{} image", value.name),
                url: category,
            }),
        }
    }
}

impl TryFrom<sellershut_core::categories::Category> for HutCategory {
    type Error = AppError;

    fn try_from(value: sellershut_core::categories::Category) -> Result<Self, Self::Error> {
        let id = Url::parse(&value.ap_id)?;
        let sub_categories: Result<Vec<HutCategory>, _> = value
            .sub_categories
            .into_iter()
            .map(|sub_category| HutCategory::try_from(sub_category))
            .collect();

        let image_url = value.image_url.map(|value| Url::parse(&value));

        Ok(Self {
            id: id.into(),
            name: value.name,
            sub_categories: sub_categories?,
            image: match image_url {
                Some(value) => Some(value?),
                None => None,
            },
        })
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
            let category = Self::try_from(resp)?;
            Ok(Some(category))
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
        let mut client = data.query_categories_client.clone();
        let sub_categories = GetAllSubCategoriesRequest::default();
        let sub_categories: Result<Vec<_>, _> = client
            .all_sub_categories(sub_categories.into_request())
            .instrument({
                let span = tracing::info_span!("grpc.call");
                span.set_attribute(trace::RPC_SERVICE, "QueryCategoriesClient");
                span.set_attribute(trace::RPC_METHOD, "AllSubCategories");
                span
            })
            .await?
            .into_inner()
            .categories
            .into_iter()
            .map(HutCategory::try_from)
            .collect();

        let sub_categories = sub_categories?.into_iter().map(Category::from).collect();

        Ok(Self::Kind {
            id: self.id.clone(),
            kind: Default::default(),
            name: self.name.clone(),
            total_items: self.sub_categories.len(),
            items: sub_categories,
            image: self.image.map(|category| CategoryImage {
                kind: Default::default(),
                name: format!("{} image", self.name),
                url: category,
            }),
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

        HutCategory::try_from(resp)
    }
}
