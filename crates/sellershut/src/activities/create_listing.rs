use activitypub_federation::{
    config::Data, fetch::object_id::ObjectId, kinds::activity::CreateType,
    protocol::helpers::deserialize_one_or_many, traits::ActivityHandler,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    entities::{listing::Listing, user::LocalUser},
    state::AppState,
};

#[derive(Serialize, Deserialize)]
pub struct CreateListing {
    actor: ObjectId<LocalUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    pub(crate) object: Listing,
    #[serde(rename = "type")]
    pub(crate) kind: CreateType,
    pub(crate) id: Url,
}

impl CreateListing {
    pub fn new(listing: Listing, id: Url) -> Self {
        Self {
            actor: listing.attributed_to.clone(),
            to: listing.to.clone(),
            object: listing,
            kind: CreateType::Create,
            id,
        }
    }
}

#[tonic::async_trait]
impl ActivityHandler for CreateListing {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppState;

    #[doc = " Error type returned by handler methods"]
    type Error = tonic::Status;

    #[doc = " `id` field of the activity"]
    fn id(&self) -> &Url {
        &self.id
    }

    #[doc = " `actor` field of activity"]
    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    #[doc = " Verifies that the received activity is valid."]
    #[doc = ""]
    #[doc = " This needs to be a separate method, because it might be used for activities"]
    #[doc = " like `Undo/Follow`, which shouldn\'t perform any database write for the inner `Follow`."]
    #[must_use]
    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        todo!()
    }

    #[doc = " Called when an activity is received."]
    #[doc = ""]
    #[doc = " Should perform validation and possibly write action to the database. In case the activity"]
    #[doc = " has a nested `object` field, must call `object.from_json` handler."]
    #[must_use]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        todo!()
    }
}
