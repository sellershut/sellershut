mod follow;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::FollowType,
    traits::{ActivityHandler, Actor},
};
use axum::async_trait;
use sellershut_core::users::FollowUserRequest;
use serde::{Deserialize, Serialize};
use tonic::IntoRequest;
use tracing::{Instrument, info_span};
use url::Url;

use crate::{
    generate_object_id,
    hut::{Hut, activities::accept::Accept, entities::HutUser},
    server::error::AppError,
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

#[async_trait]
impl ActivityHandler for Follow {
    type DataType = Hut;
    type Error = AppError;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let mut client = data.mutate_users_client.clone();
        let follow_request = FollowUserRequest {
            url: self.actor.clone().into_inner().to_string(),
            follow_url: self.object.clone().into_inner().to_string(),
        }
        .into_request();

        let resp = client
            .follow_user(follow_request)
            .instrument(info_span!("grpc.user.follow"))
            .await?
            .into_inner()
            .user;

        let user = HutUser(resp);

        let follower = self.actor.dereference(data).await?;
        let id = generate_object_id(data.domain())?;

        let accept = Accept::new(user.id()?, self, id.clone());

        user.send(accept, vec![follower.shared_inbox_or_inbox()], true, data)
            .await?;

        Ok(())
    }
}
