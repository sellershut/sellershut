use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object},
};
use sellershut_core::user::ActorType;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::{
    PartialSchema, ToSchema,
    openapi::{ObjectBuilder, schema::SchemaType},
};

use crate::server::{AppError, state::AppState};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    data: sellershut_core::user::User,
    id: ObjectId<User>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: ActorType,
    preferred_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[schema(value_type = String)]
    id: ObjectId<User>,
    inbox: Url,
    public_key: PublicKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<UserIcon>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIcon {
    #[serde(rename = "type")]
    kind: String,
    name: String,
    url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u16>,
}

#[async_trait::async_trait]
impl Object for User {
    type DataType = AppState;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Person;

    #[doc = " Error type returned by handler methods"]
    type Error = AppError;

    #[doc = " `id` field of the object"]
    fn id(&self) -> &Url {
        self.id.inner()
    }

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let user = data.user.get_user_by_id(&object_id).await?.map(User::from);
        Ok(user)
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind::try_from(self)?)
    }

    #[doc = " Verifies that the received object is valid."]
    #[doc = ""]
    #[doc = " You should check here that the domain of id matches `expected_domain`. Additionally you"]
    #[doc = " should perform any application specific checks."]
    #[doc = ""]
    #[doc = " It is necessary to use a separate method for this, because it might be used for activities"]
    #[doc = " like `Delete/Note`, which shouldn\'t perform any database write for the inner `Note`."]
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
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Actor for User {
    fn public_key_pem(&self) -> &str {
        &self.data.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.data.private_key.clone().map(|f| f.expose())
    }

    fn inbox(&self) -> url::Url {
        self.data.inbox.inner()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey(activitypub_federation::protocol::public_key::PublicKey);

impl ToSchema for PublicKey {}

impl PartialSchema for PublicKey {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        ObjectBuilder::new()
            .schema_type(SchemaType::new(utoipa::openapi::Type::Object))
            .property(
                "id",
                ObjectBuilder::new()
                    .schema_type(SchemaType::new(utoipa::openapi::Type::String))
                    .build(),
            )
            .property(
                "owner",
                ObjectBuilder::new()
                    .schema_type(SchemaType::new(utoipa::openapi::Type::String))
                    .build(),
            )
            .property(
                "publicKeyPem",
                ObjectBuilder::new()
                    .schema_type(SchemaType::new(utoipa::openapi::Type::String))
                    .build(),
            )
            .required("id")
            .required("owner")
            .required("publicKeyPem")
            .build()
            .into()
    }
}

impl From<sellershut_core::user::User> for User {
    fn from(value: sellershut_core::user::User) -> Self {
        let id = value.ap_id.inner().into();
        Self { data: value, id }
    }
}

impl TryFrom<User> for Person {
    type Error = url::ParseError;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        let preferred_username = value.data.username.clone();
        let icon = if let Some(avatar) = value.data.avatar.as_ref() {
            let url = Url::parse(avatar)?;

            Some(UserIcon {
                kind: "Image".to_owned(),
                name: "Profile picture".to_owned(),
                url,
                width: None,
                height: None,
            })
        } else {
            None
        };

        Ok(Self {
            kind: value.data.kind,
            preferred_username,
            id: value.id.clone(),
            inbox: value.data.inbox.inner(),
            public_key: PublicKey(value.public_key()),
            name: value.data.name,
            icon,
        })
    }
}
