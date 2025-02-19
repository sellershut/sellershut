pub mod mutation;
pub mod query;

use activitypub_federation::{
    activity_queue::queue_activity,
    activity_sending::SendActivityTask,
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{actor::PersonType, collection::CollectionType, link::LinkType},
    protocol::{context::WithContext, public_key::PublicKey, verification::verify_domains_match},
    traits::{ActivityHandler, Actor, Object},
};
use anyhow::anyhow;
use sellershut_core::users::{QueryUserByApIdRequest, UpsertUserRequest, User};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tonic::IntoRequest;
use tracing::{debug, info_span, instrument, Instrument};
use url::Url;

use crate::{
    activities::{accept::AcceptActivity, follow::FollowActivity},
    server::error::{ApiResult, AppError},
    state::AppHandle,
};

#[derive(Debug, Clone)]
pub struct HutUser(pub sellershut_core::users::User);

/// List of all activities which this actor can receive.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonAcceptedActivities {
    Follow(FollowActivity),
    Accept(AcceptActivity),
}

impl HutUser {
    pub fn id(&self) -> ApiResult<ObjectId<HutUser>> {
        let id = Url::parse(&self.0.ap_id)?;
        Ok(id.into())
    }

    pub fn followers_url(&self) -> ApiResult<Url> {
        let url = Url::parse(&self.0.ap_id).map(|mut url| {
            url.set_path("followers");
            url
        });
        Ok(url?)
    }

    pub fn following_url(&self) -> ApiResult<Url> {
        let url = Url::parse(&self.0.ap_id).map(|mut url| {
            url.set_path("following");
            url
        });
        Ok(url?)
    }

    pub(crate) async fn send<Activity>(
        &self,
        activity: Activity,
        recipients: Vec<Url>,
        use_queue: bool,
        data: &Data<AppHandle>,
    ) -> ApiResult<()>
    where
        Activity: ActivityHandler + Serialize + std::fmt::Debug + Send + Sync,
        <Activity as ActivityHandler>::Error: From<anyhow::Error> + From<serde_json::Error>,
    {
        let activity = WithContext::new_default(activity);
        // Send through queue in some cases and bypass it in others to test both code paths
        if use_queue {
            queue_activity(&activity, self, recipients, data).await?;
        } else {
            let sends = SendActivityTask::prepare(&activity, self, recipients, data).await?;
            for send in sends {
                send.sign_and_send(data).await?;
            }
        }
        Ok(())
    }
}

impl TryFrom<HutUser> for Person {
    type Error = AppError;

    fn try_from(value: HutUser) -> Result<Self, Self::Error> {
        let id = Url::parse(&value.0.ap_id)?;
        let inbox = Url::parse(&value.0.inbox)?;
        let outbox = Url::parse(&value.0.outbox)?;

        Ok(Self {
            id: id.into(),
            inbox,
            outbox,
            name: value.0.display_name.clone(),
            kind: Default::default(),
            preferred_username: value.0.username.clone(),
            public_key: value.public_key(),
            icon: match value.0.avatar_url {
                Some(ref value) => Some(Url::parse(value)?),
                None => None,
            },
            followers: value.followers_url()?,
            following: value.following_url()?,
            summary: value.0.summary,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    id: ObjectId<HutUser>,
    inbox: Url,
    outbox: Url,
    public_key: PublicKey,
    following: Url,
    followers: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    #[serde(rename = "type")]
    pub kind: CollectionType,
    pub total_items: usize,
    pub items: Vec<Follower>,
}

impl TryFrom<Vec<String>> for Follow {
    type Error = AppError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: CollectionType::Collection,
            total_items: value.len(),
            items: {
                let items: Result<Vec<_>, _> = value
                    .iter()
                    .map(|v| Url::parse(v).map(Follower::from))
                    .collect();
                items?
            },
        })
    }
}

impl From<Url> for Follower {
    fn from(value: Url) -> Self {
        Self {
            kind: LinkType::Link,
            href: value,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    #[serde(rename = "type")]
    pub kind: LinkType,
    pub href: Url,
}

#[tonic::async_trait]
impl Object for HutUser {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppHandle;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Person;

    #[doc = " Error type returned by handler methods"]
    type Error = AppError;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[instrument(skip(data), err(Debug))]
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut client = data.query_users_client.clone();
        let query_by_id = QueryUserByApIdRequest {
            ap_id: object_id.to_string(),
        };

        debug!(id = ?object_id, "getting user");

        let resp = client
            .query_user_by_ap_id(query_by_id.into_request())
            .instrument(info_span!("grpc.user.get"))
            .await?
            .into_inner()
            .user;

        if let Some(resp) = resp {
            debug!("user found {resp:?}");
            let user = HutUser(resp);
            Ok(Some(user))
        } else {
            debug!("user not found");
            Ok(None)
        }
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    #[instrument(skip(_data), err(Debug))]
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Self::Kind::try_from(self)
    }

    #[doc = " Verifies that the received object is valid."]
    #[doc = ""]
    #[doc = " You should check here that the domain of id matches `expected_domain`. Additionally you"]
    #[doc = " should perform any application specific checks."]
    #[doc = ""]
    #[doc = " It is necessary to use a separate method for this, because it might be used for activities"]
    #[doc = " like `Delete/Note`, which shouldn\'t perform any database write for the inner `Note`."]
    #[instrument(skip(_data), err(Debug))]
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
    #[instrument(skip(data), err(Debug))]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let id = json.id;
        debug!(id = ?id, "upserting user");

        let request = UpsertUserRequest {
            user: Some(User {
                username: json.preferred_username,
                ap_id: id.clone().into_inner().to_string(),
                public_key: json.public_key.public_key_pem,
                private_key: None,
                inbox: json.inbox.to_string(),
                outbox: json.outbox.to_string(),
                last_refreshed_at: Some(OffsetDateTime::now_utc().into()),
                avatar_url: json.icon.map(Into::into),
                summary: json.summary,
                ..Default::default()
            }),
        }
        .into_request();

        let mut client = data.mutate_users_client.clone();
        let resp = client
            .upsert_user(request)
            .await?
            .into_inner()
            .user
            .ok_or_else(|| anyhow!("user not returned from upsert"))?;
        debug!(id = ?id, "user upserted");

        Ok(HutUser(resp))
    }
}

impl Actor for HutUser {
    fn id(&self) -> Url {
        Url::parse(&self.0.ap_id).expect("ap id to be url")
    }

    fn public_key_pem(&self) -> &str {
        &self.0.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.0.private_key.clone()
    }

    fn inbox(&self) -> Url {
        Url::parse(&self.0.inbox).expect("inbox to be url")
    }
}
