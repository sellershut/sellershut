use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{actor::PersonType, collection::CollectionType, link::LinkType},
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use sellershut_core::users::{
    QueryUserByIdRequest, QueryUsersFollowingRequest, UpsertUserRequest, User,
};
use serde::{de::value, Deserialize, Serialize};
use time::OffsetDateTime;
use tonic::IntoRequest;
use tracing::{debug, info_span, instrument, Instrument};
use url::Url;

use crate::{
    server::error::{ApiResult, AppError},
    state::AppHandle,
};

#[derive(Debug, Clone)]
pub struct HutUser(pub sellershut_core::users::User);

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
    following: Follow,
    followers: Follow,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    #[serde(rename = "type")]
    kind: CollectionType,
    total_items: usize,
    items: Vec<Follower>,
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
    kind: LinkType,
    href: Url,
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
        let query_by_id = QueryUserByIdRequest {
            id: object_id.to_string(),
        };

        debug!(id = ?object_id, "getting user");

        let resp = client
            .query_user_by_id(query_by_id.into_request())
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
    #[instrument(skip(data), err(Debug))]
    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let id = Url::parse(&self.0.ap_id)?;
        let inbox = Url::parse(&self.0.inbox)?;
        let outbox = Url::parse(&self.0.outbox)?;

        let followers: Result<Vec<_>, _> = self
            .0
            .followers
            .iter()
            .map(|v| {
                let url = Url::parse(v);
                url.map(Follower::from)
            })
            .collect();

        let followers = followers?;

        let mut query = data.query_users_client.clone();
        let following = query
            .query_user_following(
                QueryUsersFollowingRequest {
                    id: self.0.ap_id.to_string(),
                }
                .into_request(),
            )
            .await?;
        let following: Result<Vec<_>, _> = following
            .into_inner()
            .users
            .iter()
            .map(|value| Url::parse(value).map(Follower::from))
            .collect();

        let following = following?;

        Ok(Self::Kind {
            id: id.into(),
            inbox,
            outbox,
            name: self.0.display_name.clone(),
            kind: Default::default(),
            preferred_username: self.0.username.clone(),
            public_key: self.public_key(),
            icon: match self.0.avatar_url {
                Some(ref value) => Some(Url::parse(value)?),
                None => None,
            },
            summary: self.0.summary,
            followers: Follow {
                kind: CollectionType::Collection,
                total_items: followers.len(),
                items: followers,
            },
            following: Follow {
                kind: CollectionType::Collection,
                total_items: following.len(),
                items: following,
            },
        })
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
            user: User {
                username: json.preferred_username,
                ap_id: id.clone().into_inner().to_string(),
                public_key: json.public_key.public_key_pem,
                private_key: None,
                inbox: json.inbox.to_string(),
                outbox: json.outbox.to_string(),
                last_refreshed_at: OffsetDateTime::now_utc().into(),
                avatar_url: json.icon.map(Into::into),
                summary: json.summary,
                followers: json
                    .followers
                    .items
                    .into_iter()
                    .map(|value| value.href.to_string())
                    .collect(),
                ..Default::default()
            },
        }
        .into_request();

        let mut client = data.mutate_users_client.clone();
        let resp = client.upsert_user(request).await?.into_inner().user;
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
