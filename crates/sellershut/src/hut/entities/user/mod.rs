mod activities;
pub use activities::*;

use crate::Hut;
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::PersonType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use chrono::{DateTime, Utc};
use sellershut_core::users::{QueryUserByIdRequest, UpsertUserRequest, User};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tonic::{IntoRequest, async_trait};
use tracing::{Instrument, info_span};
use url::Url;

use crate::server::error::AppError;

#[derive(Debug, Clone)]
pub struct HutUser {
    pub id: ObjectId<HutUser>,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<Url>,
    pub email: Option<String>,
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
            display_name: value.display_name,
            email: value.email,
            followers,
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
    name: Option<String>,
    id: ObjectId<HutUser>,
    inbox: Url,
    public_key: PublicKey,
}

#[async_trait]
impl Object for HutUser {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = Hut;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Person;

    #[doc = " Error type returned by handler methods"]
    type Error = AppError;

    fn last_refreshed_at(&self) -> Option<DateTime<Utc>> {
        let dt = self.last_refreshed_at.unix_timestamp();
        DateTime::from_timestamp_millis(dt)
    }

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut client = data.query_users_client.clone();
        let query_by_id = QueryUserByIdRequest {
            id: object_id.to_string(),
        };
        let resp = client
            .query_user_by_id(query_by_id.into_request())
            .await?
            .into_inner()
            .user;

        if let Some(resp) = resp {
            let user = Self::try_from(resp)?;
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    #[must_use]
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Self::Kind {
            id: self.id.clone(),
            inbox: self.inbox.clone(),
            name: self.display_name.clone(),
            kind: Default::default(),
            preferred_username: self.username.clone(),
            public_key: self.public_key(),
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
        let request = UpsertUserRequest {
            user: User {
                username: json.preferred_username,
                id: json.id.into_inner().to_string(),
                public_key: json.public_key.public_key_pem,
                private_key: None,
                inbox: json.inbox.to_string(),
                last_refreshed_at: OffsetDateTime::now_utc().into(),
                ..Default::default()
            },
        }
        .into_request();

        let mut client = data.mutate_users_client.clone();
        let resp = client
            .upsert_user(request)
            .instrument(info_span!("grpc.user.upsert"))
            .await?
            .into_inner()
            .user;

        HutUser::try_from(resp)
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
