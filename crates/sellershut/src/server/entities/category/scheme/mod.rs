use activitypub_federation::{
    config::Data,
    error::Error as FederationError,
    fetch::object_id::ObjectId,
    protocol::verification::verify_domains_match,
    traits::{Activity, Object},
};
use async_trait::async_trait;
use sellershut_categories::UpsertCategoryScheme;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use url::Url;
use utoipa::ToSchema;

use crate::server::{
    AppError, activities::categories::scheme::create::CreateCategoryScheme, entities::user::User,
    state::AppState, utilities::ActivityPubIds,
};

type CoreCategoryScheme = sellershut_core::category::CategoryScheme;

const SKOS_CONCEPT_SCHEME: &str = "skos:ConceptScheme";
const SKOS_CONCEPT_SCHEME_IRI: &str = "http://www.w3.org/2004/02/skos/core#ConceptScheme";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryScheme {
    data: CoreCategoryScheme,
    id: ObjectId<CategoryScheme>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FederatedCategoryScheme {
    #[serde(rename = "@context")]
    pub context: Option<Value>,
    #[schema(value_type = String)]
    pub id: ObjectId<CategoryScheme>,
    #[serde(rename = "type")]
    pub kind: Vec<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_concepts: Option<Url>,
    #[schema(value_type = String)]
    pub attributed_to: ObjectId<User>,
    #[serde(with = "time::serde::rfc3339")]
    pub published: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
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
        let owner = self.data.owner_ap_id.clone().inner();
        let owner = ObjectId::parse(owner.as_str())?;

        Ok(FederatedCategoryScheme {
            context: Some(category_scheme_context(data.domain())?),
            id: self.id,
            kind: vec!["Object".to_owned(), "skos:ConceptScheme".to_owned()],
            name: self.data.name.to_string(),
            top_concepts: Some(top_concepts),
            attributed_to: owner,
            published: self.data.created_at,
            updated: self.data.updated_at,
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        dbg!(&json.id.inner().as_str(), expected_domain.as_str());
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

        let user = json.attributed_to.dereference(data).await?;
        //dbg!(&user.id().as_str(), json.id.inner().as_str());
        verify_domains_match(user.id(), json.id.inner())?;

        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let ap_id = json.id.inner();
        let top_concepts_ap_id = json.top_concepts.as_ref().map(Url::as_str);
        let owner_ap_id = json.attributed_to.inner().as_str();

        let c = UpsertCategoryScheme {
            ap_id,
            name: &json.name,
            owner_ap_id,
            top_concepts_ap_id,
            is_local: false,
        };

        let scheme = data.category.upsert_scheme(&c).await?;

        let actor = json.attributed_to.inner().clone();

        CreateCategoryScheme::send(json, actor, data).await?;

        Ok(scheme.into())
    }
}

impl From<CoreCategoryScheme> for CategoryScheme {
    fn from(value: CoreCategoryScheme) -> Self {
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

#[async_trait]
impl Activity for FederatedCategoryScheme {
    type DataType = AppState;

    type Error = AppError;

    fn id(&self) -> &Url {
        self.id.inner()
    }

    fn actor(&self) -> &Url {
        self.attributed_to.inner()
    }

    #[doc = " Verifies that the received activity is valid."]
    #[doc = ""]
    #[doc = " This needs to be a separate method, because it might be used for activities"]
    #[doc = " like `Undo/Follow`, which shouldn\'t perform any database write for the inner `Follow`."]
    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let url = Url::parse(data.domain())?;
        CategoryScheme::verify(self, &url, data).await
    }

    #[doc = " Called when an activity is received."]
    #[doc = ""]
    #[doc = " Should perform validation and possibly write action to the database. In case the activity"]
    #[doc = " has a nested `object` field, must call `object.from_json` handler."]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let user = self.attributed_to.dereference(data).await?;
        let ap_ids = ActivityPubIds::new(data.port, data.domain(), user.name())?;

        let scheme = CreateCategoryScheme::new(self, ap_ids.activity()?);
        scheme.receive(data).await?;

        Ok(())
    }
}
