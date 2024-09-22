use crate::state::AppState;
use activitypub_federation::{config::Data, fetch::object_id::ObjectId, traits::Object};
use sellershut_core::users::{CreateUserRequest, User as DbUser};
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct User(DbUser);

impl User {
    pub fn get(&self) -> &DbUser {
        &self.0
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalUser {
    id: ObjectId<User>,
}

#[tonic::async_trait]
impl Object for User {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppState;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = LocalUser;

    #[doc = " Error type returned by handler methods"]
    type Error = tonic::Status;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
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
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
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
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let user = DbUser {
            id: json.id.to_string(),
            ..Default::default()
        };
        let request = CreateUserRequest { user: Some(user) };

        let mut client = data.mutate_users_client.clone();

        let response = client
            .create_user(request.into_request())
            .await?
            .into_inner()
            .user;

        let user = response.expect("user");
        Ok(User(user))
    }
}
