use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{
        collection::{CollectionPageType, CollectionType},
        link::LinkType,
        object::ImageType,
    },
    protocol::verification::verify_domains_match,
    traits::Object,
};
use anyhow::anyhow;
use sellershut_core::categories::{GetCategoryRequest, SubCategory, UpsertCategoryRequest};
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use tracing::{debug, info_span, Instrument};
use url::Url;

use crate::{server::error::AppError, state::AppHandle};

#[derive(Debug, Clone)]
pub struct HutCategory(CategoryType);

#[derive(Debug, Clone)]
pub enum CategoryType {
    Simple(sellershut_core::categories::Category),
    Detailed(sellershut_core::categories::CategoryDetailed),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "type")]
    kind: CollectionType,
    id: ObjectId<HutCategory>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<CategoryImage>,
    total_items: usize,
    // subcategories
    first: CategoryPagination,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryImage {
    #[serde(rename = "type")]
    kind: ImageType,
    name: String,
    url: Url,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryPagination {
    #[serde(rename = "type")]
    kind: CollectionPageType,
    part_of: ObjectId<HutCategory>,
    items: Vec<CategoryItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryItem {
    #[serde(rename = "type")]
    kind: LinkType,
    name: String,
    href: Url,
}

impl TryFrom<SubCategory> for CategoryItem {
    type Error = AppError;

    fn try_from(value: SubCategory) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: LinkType::Link,
            name: value.name,
            href: Url::parse(&value.ap_id)?,
        })
    }
}

impl TryFrom<HutCategory> for Category {
    type Error = AppError;

    fn try_from(value: HutCategory) -> Result<Self, Self::Error> {
        if let CategoryType::Detailed(value) = value.0 {
            let id: ObjectId<HutCategory> = Url::parse(&value.ap_id)?.into();

            Ok(Self {
                kind: CollectionType::Collection,
                id: id.clone(),
                name: value.name.clone(),
                total_items: value.sub_categories.len(),
                image: match value.image_url {
                    Some(url) => {
                        let url = Url::parse(&url)?;
                        Some(CategoryImage {
                            kind: ImageType::Image,
                            name: value.name,
                            url,
                        })
                    }
                    None => None,
                },
                first: CategoryPagination {
                    kind: CollectionPageType::CollectionPage,
                    part_of: id,
                    items: {
                        let items: Result<Vec<_>, _> = value
                            .sub_categories
                            .into_iter()
                            .map(TryFrom::try_from)
                            .collect();
                        items?
                    },
                },
            })
        } else {
            todo!("this should be an error, can't call with this type of category")
        }
    }
}

#[tonic::async_trait]
impl Object for HutCategory {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppHandle;

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
        let query_by_id = GetCategoryRequest {
            ap_id: object_id.to_string(),
        };

        debug!(id = ?object_id, "getting category");

        let resp = client
            .category_by_ap_id(query_by_id.into_request())
            .instrument(info_span!("grpc.category.get"))
            .await?
            .into_inner()
            .category;

        if let Some(resp) = resp {
            debug!("category found {resp:?}");
            let category = HutCategory(CategoryType::Detailed(resp));
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
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Self::Kind::try_from(self)
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
        debug!(id = ?id, "upserting category");

        let request = UpsertCategoryRequest {
            category: Some(sellershut_core::categories::Category {
                name: json.name,
                sub_categories: json
                    .first
                    .items
                    .iter()
                    .map(|v| v.href.to_string())
                    .collect(),
                image_url: json.image.map(|v| v.url.to_string()),
                // TODO: wtf            parent_id: todo!(),
                ap_id: id.clone().into_inner().to_string(),
                ..Default::default()
            }),
        }
        .into_request();

        let mut client = data.mutate_categories_client.clone();
        let resp = client
            .upsert(request)
            .await?
            .into_inner()
            .category
            .ok_or_else(|| anyhow!("category not returned from upsert"))?;
        debug!(id = ?id, "category upserted");

        Ok(Self(CategoryType::Simple(resp)))
    }
}
