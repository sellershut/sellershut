use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::FollowType,
    traits::{ActivityHandler, Actor},
};
use sellershut_core::users::FollowUserRequest;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activities::accept::Accept, entities::user::HutUser, server::error::AppError, state::AppHandle,
    utils::generate_object_id,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub(crate) actor: ObjectId<HutUser>,
    pub(crate) object: ObjectId<HutUser>,
    #[serde(rename = "type")]
    kind: FollowType,
    id: Url,
}

impl Follow {
    pub fn new(actor: ObjectId<HutUser>, object: ObjectId<HutUser>, id: Url) -> Follow {
        Follow {
            actor,
            object,
            kind: Default::default(),
            id,
        }
    }
}

#[tonic::async_trait]
impl ActivityHandler for Follow {
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
    #[must_use]
    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    #[doc = " Called when an activity is received."]
    #[doc = ""]
    #[doc = " Should perform validation and possibly write action to the database. In case the activity"]
    #[doc = " has a nested `object` field, must call `object.from_json` handler."]
    #[must_use]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let id = self.object.inner().to_string();

        let mut client = data.mutate_users_client.clone();
        let r = client
            .follow_user(FollowUserRequest {
                ap_id: id,
                follow_url: self.actor.inner().to_string(),
            })
            .await?
            .into_inner()
            .user
            .ok_or_else(|| anyhow::anyhow!("no user found in database"))?;

        let local_user = HutUser(r);

        let follower = self.actor.dereference(data).await?;
        let id = generate_object_id(data.domain(), 21)?;

        let follower_inbox = follower.shared_inbox_or_inbox();
        let accept = Accept::new(local_user.id()?, self, id.clone());

        local_user
            .send(accept, vec![follower_inbox], false, data)
            .await?;

        Ok(())
    }
}
