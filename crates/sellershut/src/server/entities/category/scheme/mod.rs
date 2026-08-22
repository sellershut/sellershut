use activitypub_federation::{
    config::Data, error::Error as FederationError, fetch::object_id::ObjectId,
    protocol::verification::verify_domains_match, traits::Object,
};
use sellershut_categories::UpsertCategoryScheme;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use time::OffsetDateTime;
use url::Url;
use uuid::Uuid;

use crate::server::{AppError, entities::user::User, state::AppState};

type CoreCategoryScheme = sellershut_core::category::CategoryScheme;

const SKOS_CONCEPT_SCHEME: &str = "skos:ConceptScheme";
const SKOS_CONCEPT_SCHEME_IRI: &str = "http://www.w3.org/2004/02/skos/core#ConceptScheme";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryScheme {
    data: CoreCategoryScheme,
    id: ObjectId<CategoryScheme>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FederatedCategoryScheme {
    #[serde(rename = "@context")]
    pub context: Value,
    pub id: ObjectId<CategoryScheme>,
    #[serde(rename = "type")]
    pub kind: Vec<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_concepts: Option<Url>,
    pub attributed_to: ObjectId<User>,
    pub published: OffsetDateTime,
    pub updated: OffsetDateTime,
}

fn category_scheme_context(domain: &str) -> Result<serde_json::Value, AppError> {
    let scheme = if cfg!(debug_assertions) {
        "http"
    } else {
        "https"
    };

    let mut url = Url::parse(&format!("{scheme}://{domain}"))?;
    url.set_path("ns");
    url.set_fragment(Some("topConcepts"));

    Ok(serde_json::json!([
        "https://www.w3.org/ns/activitystreams",
        {
            "skos": "http://www.w3.org/2004/02/skos/core#",
            "topConcepts": {
                "@id": url.to_string(),
                "@type": "@id"
            }
        }
    ]))
}

fn is_concept_scheme(types: &[String]) -> bool {
    types
        .iter()
        .any(|kind| kind == SKOS_CONCEPT_SCHEME || kind == SKOS_CONCEPT_SCHEME_IRI)
}

fn invalid_federated_object(message: impl Into<String>) -> AppError {
    FederationError::Other(message.into()).into()
}

#[async_trait::async_trait]
impl Object for CategoryScheme {
    type DataType = AppState;
    type Kind = FederatedCategoryScheme;
    type Error = AppError;

    fn id(&self) -> &Url {
        self.id.inner()
    }

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let scheme = data.category.get_category_scheme(&object_id).await?;
        Ok(scheme.map(Into::into))
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let top_concepts = append_path(self.id.inner(), "top-concepts");

        Ok(FederatedCategoryScheme {
            context: category_scheme_context(data.domain())?,
            id: self.id,
            kind: vec!["Object".to_owned(), "skos:ConceptScheme".to_owned()],
            name: self.data.name.to_string(),
            top_concepts: Some(top_concepts),
            attributed_to: self.data.owner_ap_id.map(Into::into),
            published: self.data.created_at,
            updated: self.data.updated_at,
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;

        if !is_concept_scheme(&json.kind) {
            return Err(invalid_federated_object(
                "object is not a skos:ConceptScheme",
            ));
        }

        if json.name.trim().is_empty() {
            return Err(invalid_federated_object(
                "category scheme name must not be empty",
            ));
        }

        if json.updated < json.published {
            return Err(invalid_federated_object(
                "category scheme updated timestamp is before published timestamp",
            ));
        }

        if let Some(owner_ap_id) = &json.attributed_to {
            // This is an application-specific ownership rule:
            // require the owner actor to be hosted on the same domain
            // as the scheme.
            //
            // Remove this check if you deliberately support an actor
            // on one domain owning a scheme on another domain.
            verify_domains_match(&owner_ap_id, json.id.inner())?;
        }

        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let FederatedCategoryScheme {
            context: _,
            id,
            kind: _,
            name,
            top_concepts,
            attributed_to,
            published,
            updated,
        } = json;

        let ap_id = id.inner();
        let top_concepts_ap_id = top_concepts.as_ref().map(Url::as_str);
        let owner_ap_id = attributed_to.as_ref().map(Url::as_str);

        let c = UpsertCategoryScheme {
            ap_id,
            name: &name,
            owner_ap_id,
            top_concepts_ap_id,
            is_local: false,
            ap_published_at: Some(&published),
            ap_updated_at: Some(&updated),
        };
        let scheme = data.category.upsert_scheme(&c).await?;

        Ok(scheme.into())
    }
}

impl From<CoreCategoryScheme> for CategoryScheme {
    fn from(value: CoreCategoryScheme) -> Self {
        // ObjectId implements From<Url>. If ap_id.inner() returns &Url,
        // clone the Url before converting it.
        let id = ObjectId::from(value.ap_id.inner().clone());

        Self { data: value, id }
    }
}

fn append_path(base: &Url, segment: &str) -> Url {
    let mut url = base.clone();

    let path = format!(
        "{}/{}",
        url.path().trim_end_matches('/'),
        segment.trim_matches('/'),
    );

    url.set_path(&path);
    url.set_query(None);
    url.set_fragment(None);

    url
}
