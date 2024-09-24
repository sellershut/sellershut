use std::str::FromStr;

use crate::state::AppState;
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::PersonType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use sellershut_core::users::{QueryUserByApIdRequest, UpsertUserRequest, User as DbUser};
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use tracing::{info_span, instrument, Instrument};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalUser(DbUser);

impl From<DbUser> for LocalUser {
    fn from(value: DbUser) -> Self {
        Self(value)
    }
}

impl LocalUser {
    pub fn get(&self) -> &DbUser {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    id: ObjectId<LocalUser>,
    inbox: Url,
    public_key: PublicKey,
}

#[tonic::async_trait]
impl Object for LocalUser {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppState;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = User;

    #[doc = " Error type returned by handler methods"]
    type Error = tonic::Status;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
    #[instrument(skip(data))]
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut client = data.query_users_client.clone();
        let query = QueryUserByApIdRequest {
            ap_id: object_id.to_string(),
        };

        let response = client
            .query_user_by_ap_id(query.into_request())
            .instrument(info_span!("grpc.user.by.apid"))
            .await?
            .into_inner()
            .user;

        Ok(response.map(Self::from))
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    #[must_use]
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let id: ObjectId<LocalUser> = ObjectId::from_str(&self.0.id).unwrap();
        Ok(User {
            preferred_username: self.0.username.clone(),
            kind: Default::default(),
            id,
            inbox: Url::from_str(&self.0.inbox).unwrap(),
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
        verify_domains_match(json.id.inner(), expected_domain)
            .map_err(|_e| tonic::Status::failed_precondition("domains do not match"))?;
        Ok(())
    }

    #[doc = " Convert object from ActivityPub type to database type."]
    #[doc = ""]
    #[doc = " Called when an object is received from HTTP fetch or as part of an activity. This method"]
    #[doc = " should write the received object to database. Note that there is no distinction between"]
    #[doc = " create and update, so an `upsert` operation should be used."]
    #[must_use]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let user = DbUser {
            id: json.id.to_string(),
            ..Default::default()
        };
        let request = UpsertUserRequest { user: Some(user) };

        let mut client = data.mutate_users_client.clone();

        let response = client
            .upsert_user(request.into_request())
            .instrument(info_span!("rpc-upsert-user"))
            .await?
            .into_inner()
            .user;

        let user = response.expect("user");
        Ok(LocalUser(user))
    }
}

impl Actor for LocalUser {
    fn id(&self) -> Url {
        Url::from_str(&self.0.id).unwrap()
    }

    fn public_key_pem(&self) -> &str {
        &self.0.public_key_pem
    }

    fn private_key_pem(&self) -> Option<String> {
        self.0.private_key_pem.clone()
    }

    fn inbox(&self) -> Url {
        Url::from_str(&self.0.inbox).unwrap()
    }
}
