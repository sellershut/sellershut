use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::CreateType,
    protocol::helpers::deserialize_one_or_many,
    traits::{ActivityHandler, Object},
};
use axum::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    hut::{
        Hut,
        entities::{HutListing, HutUser, Listing},
    },
    server::error::AppError,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateListing {
    pub(crate) actor: ObjectId<HutUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    pub(crate) object: Listing,
    #[serde(rename = "type")]
    pub(crate) kind: CreateType,
    pub(crate) id: Url,
}

impl CreateListing {
    pub fn new(listing: Listing, id: Url) -> CreateListing {
        CreateListing {
            actor: listing.attributed_to.clone(),
            to: listing.to.clone(),
            object: listing,
            kind: CreateType::Create,
            id,
        }
    }
}

#[async_trait]
impl ActivityHandler for CreateListing {
    type DataType = Hut;
    type Error = AppError;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        HutListing::verify(&self.object, &self.id, data).await?;
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        HutListing::from_json(self.object, data).await?;
        Ok(())
    }
}
