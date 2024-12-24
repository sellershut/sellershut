use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::PersonType,
    protocol::public_key::PublicKey,
    traits::{ActivityHandler, Actor, Object},
};
use sellershut_core::users::User;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tonic::async_trait;
use url::Url;

use crate::server::error::AppError;

use super::{Hut, activities::follow::Follow};

#[derive(Debug, Clone)]
pub struct HutUser {
    pub id: ObjectId<HutUser>,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<Url>,
    pub followers: Vec<Url>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_refreshed_at: OffsetDateTime,
    pub inbox: Url,
    pub public_key: String,
    pub private_key: Option<String>,
    pub local: bool,
}

impl TryFrom<User> for HutUser {
    type Error = AppError;
    fn try_from(value: User) -> Result<Self, Self::Error> {
        let id = Url::parse(&value.id)?;
        let avatar_url = if let Some(ref url) = value.avatar_url {
            Some(Url::parse(url)?)
        } else {
            None
        };
        let followers: Result<Vec<_>, _> = value.followers.iter().map(|f| Url::parse(f)).collect();
        let followers = followers?;

        Ok(Self {
            id: id.into(),
            avatar_url,
            created_at: value.created_at.try_into()?,
            last_refreshed_at: value.last_refreshed_at.try_into()?,
            updated_at: value.updated_at.try_into()?,
            username: value.username,
            local: value.local,
            private_key: value.private_key,
            public_key: value.public_key,
            followers,
            email: value.email,
            inbox: Url::parse(&value.inbox)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    id: ObjectId<HutUser>,
    inbox: Url,
    public_key: PublicKey,
}

/// List of all activities which this actor can receive.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonAcceptedActivities {
    Follow(Follow),
}

#[async_trait]
impl Object for HutUser {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = Hut;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Person;

    #[doc = " Error type returned by handler methods"]
    type Error = anyhow::Error;

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
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
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

impl Actor for HutUser {
    fn id(&self) -> Url {
        self.id.inner().clone()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        self.inbox.clone()
    }
}
