use activitypub_federation::{
    config::Data, fetch::object_id::ObjectId, kinds::activity::AcceptType, traits::ActivityHandler,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{entities::user::HutUser, server::error::AppError, state::AppHandle};

use super::follow::FollowActivity;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AcceptActivity {
    actor: ObjectId<HutUser>,
    object: FollowActivity,
    #[serde(rename = "type")]
    kind: AcceptType,
    id: Url,
}

impl AcceptActivity {
    pub fn new(actor: ObjectId<HutUser>, object: FollowActivity, id: Url) -> AcceptActivity {
        AcceptActivity {
            actor,
            object,
            kind: Default::default(),
            id,
        }
    }
}

#[tonic::async_trait]
impl ActivityHandler for AcceptActivity {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppHandle;

    #[doc = " Error type returned by handler methods"]
    type Error = AppError;

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
    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    #[doc = " Called when an activity is received."]
    #[doc = ""]
    #[doc = " Should perform validation and possibly write action to the database. In case the activity"]
    #[doc = " has a nested `object` field, must call `object.from_json` handler."]
    async fn receive(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }
}
