use activitypub_federation::{
    activity_sending::SendActivityTask,
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::CreateType,
    protocol::{context::WithContext, helpers::deserialize_one_or_many},
    traits::{Activity, Actor, Object},
};
use async_trait::async_trait;
use sellershut_categories::UpsertCategoryScheme;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::server::{
    AppError,
    entities::{
        category::scheme::{CategoryScheme, FederatedCategoryScheme},
        user::User,
    },
    state::AppState,
    utilities::ActivityPubIds,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryScheme {
    pub(crate) actor: ObjectId<User>,
    //   #[serde(deserialize_with = "deserialize_one_or_many")]
    //   pub(crate) to: Vec<Url>,
    #[serde(
        default,
        deserialize_with = "deserialize_one_or_many",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub(crate) cc: Vec<Url>,
    pub(crate) object: FederatedCategoryScheme,
    #[serde(rename = "type")]
    pub(crate) kind: CreateType,
    pub(crate) id: Url,
}

impl CreateCategoryScheme {
    pub fn new(object: FederatedCategoryScheme, id: Url) -> Self {
        let value = object.clone();
        Self {
            actor: value.attributed_to,
            //            to: vec![activitypub_federation::kinds::public()],
            object,
            kind: CreateType::Create,
            id,
            cc: vec![],
        }
    }

    pub async fn send(
        json: FederatedCategoryScheme,
        inbox: Url,
        data: &Data<AppState>,
    ) -> Result<(), AppError> {
        let user = json.attributed_to.dereference(data).await?;

        let inbox = user.shared_inbox_or_inbox();

        let id = ActivityPubIds::new(data.port, data.domain(), user.name())?;

        let create = CreateCategoryScheme::new(json, id.activity()?);
        let create_with_context = WithContext::new_default(create);
        let sends =
            SendActivityTask::prepare(&create_with_context, &user, vec![inbox], data).await?;
        for send in sends {
            send.sign_and_send(data).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl Activity for CreateCategoryScheme {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppState;

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
    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        //CategoryScheme::verify(&self.object, &self.id, data).await?;
        Ok(())
    }

    #[doc = " Called when an activity is received."]
    #[doc = ""]
    #[doc = " Should perform validation and possibly write action to the database. In case the activity"]
    #[doc = " has a nested `object` field, must call `object.from_json` handler."]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        CategoryScheme::from_json(self.object, data).await?;
        Ok(())
    }
}
