use activitypub_federation::{
    config::Data, fetch::object_id::ObjectId, kinds::activity::AcceptType, traits::ActivityHandler,
};
use axum::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    hut::{Hut, entities::HutUser},
    server::error::AppError,
};

use super::follow::Follow;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accept {
    actor: ObjectId<HutUser>,
    object: Follow,
    #[serde(rename = "type")]
    kind: AcceptType,
    id: Url,
}

impl Accept {
    pub fn new(actor: ObjectId<HutUser>, object: Follow, id: Url) -> Accept {
        Accept {
            actor,
            object,
            kind: Default::default(),
            id,
        }
    }
}

#[async_trait]
impl ActivityHandler for Accept {
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

    async fn receive(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }
}
