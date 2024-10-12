use std::str::FromStr;

use crate::state::{cache::CacheKey, AppState};
use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    http_signatures::generate_actor_keypair,
    kinds::actor::PersonType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use infra::services::cache::{PoolLike, PooledConnectionLike};
use secrecy::{ExposeSecret, SecretString};
use sellershut_utils::id::generate_id;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::{info_span, instrument, Instrument};
use url::Url;

use super::write_to_cache;

#[derive(Debug, Deserialize)]
pub struct LocalUser {
    pub id: String,
    pub public_key: String,
    pub private_key: Option<SecretString>,
    pub inbox: Url,
    pub local: bool,
    pub ap_id: String,
    pub last_refreshed_at: OffsetDateTime,
    pub username: String,
    pub followers: Vec<Url>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// sqlx stuff
#[derive(Debug, Deserialize, Serialize)]
pub struct DbUser {
    pub id: String,
    pub username: String,
    pub last_refreshed_at: OffsetDateTime,
    pub private_key: Option<String>,
    pub public_key: String,
    pub inbox: String,
    pub followers: Vec<String>,
    pub local: bool,
    pub ap_id: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TryFrom<DbUser> for LocalUser {
    type Error = anyhow::Error;

    fn try_from(value: DbUser) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            ap_id: value.ap_id,
            public_key: value.public_key,
            private_key: value.private_key.map(SecretString::from),
            created_at: value.created_at,
            updated_at: value.updated_at,
            last_refreshed_at: value.last_refreshed_at,
            inbox: Url::from_str(value.inbox.as_str())?,
            local: value.local,
            followers: {
                let res = value
                    .followers
                    .iter()
                    .map(|value| Url::from_str(value.as_str()))
                    .collect::<Result<Vec<_>, _>>()?;
                res
            },
            username: value.username,
        })
    }
}

impl LocalUser {
    pub fn new(hostname: &str, username: &str) -> anyhow::Result<Self> {
        let id = generate_id();
        let ap_id = Url::parse(&format!("https://{}/{}", hostname, &id))?.into();
        let inbox = Url::parse(&format!("https://{}/{}/inbox", hostname, &id))?;
        let keypair = generate_actor_keypair()?;
        let ts = OffsetDateTime::now_utc();
        Ok(Self {
            id,
            ap_id,
            inbox,
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key.into()),
            last_refreshed_at: ts.clone(),
            created_at: ts.clone(),
            updated_at: ts.clone(),
            username: username.into(),
            followers: vec![],
            local: true,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,
    preferred_username: String,
    id: ObjectId<LocalUser>,
    inbox: Url,
    public_key: PublicKey,
}

#[tonic::async_trait]
impl Object for LocalUser {
    #[doc = " App data type passed to handlers. Must be identical to"]
    #[doc = " [crate::config::FederationConfigBuilder::app_data] type."]
    type DataType = AppState;

    #[doc = " The type of protocol struct which gets sent over network to federate this database struct."]
    type Kind = Person;

    #[doc = " Error type returned by handler methods"]
    type Error = anyhow::Error;

    #[doc = " Try to read the object with given `id` from local database."]
    #[doc = ""]
    #[doc = " Should return `Ok(None)` if not found."]
    #[must_use]
    #[instrument(skip(data))]
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        let cache_key = CacheKey::UserById(&object_id);
        let mut cache = data.services.cache.get().await?;

        let results = cache
            .get::<_, Vec<u8>>(cache_key)
            .instrument(info_span!("cache.get.user"))
            .await
            .and_then(|payload: Vec<u8>| {
                Ok(bincode::deserialize::<DbUser>(&payload).map(LocalUser::try_from))
            });
        match results {
            Ok(Ok(Ok(data))) => Ok(Some(data)),
            _ => {
                let db = &data.services.postgres;
                let id = object_id.to_string();
                let result = sqlx::query_as!(
                    DbUser,
                    r#"select * from federated_user where username = $1"#,
                    id
                )
                .fetch_optional(db)
                .instrument(info_span!("db.get.user"))
                .await?;

                match result {
                    Some(data) => {
                        write_to_cache::<()>(cache_key, &data, cache).await?;

                        let payload = LocalUser::try_from(data)?;
                        Ok(Some(payload))
                    }
                    None => Ok(None),
                }
            }
        }
    }

    #[doc = " Convert database type to Activitypub type."]
    #[doc = ""]
    #[doc = " Called when a local object gets fetched by another instance over HTTP, or when an object"]
    #[doc = " gets sent in an activity."]
    #[must_use]
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let ap_id = Url::from_str(&self.ap_id)?.into();

        Ok(Person {
            preferred_username: self.username.clone(),
            kind: Default::default(),
            id: ap_id,
            inbox: self.inbox.clone(),
            public_key: self.public_key(),
        })
    }

    #[doc = " Verifies that the received object is valid."]
    #[doc = ""]
    #[doc = " You should check here that the domain of id matches `expected_domain`. Additionally you"]
    #[doc = " should perform any application specific checks."]
    #[doc = ""]
    #[doc = " It is necessary to use a separate method for this, because it might be used for activities"]
    #[doc = " like `Delete/Note`, which shouldn\'t perform any database write for the inner `Note`."]
    #[must_use]
    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)
            .map_err(|_e| tonic::Status::failed_precondition("domains do not match"))?;
        Ok(())
    }

    #[doc = " Convert object from ActivityPub type to database type."]
    #[doc = ""]
    #[doc = " Called when an object is received from HTTP fetch or as part of an activity. This method"]
    #[doc = " should write the received object to database. Note that there is no distinction between"]
    #[doc = " create and update, so an `upsert` operation should be used."]
    #[must_use]
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let id = &json.id.into_inner();
        let cache_key = CacheKey::UserById(id);
        let cache = data.services.cache.get().await?;

        let db = &data.services.postgres;

        let ap_id_str = id.as_str();
        let id = ap_id_str.split("/").last().expect("id token from url");

        let username = json.preferred_username;

        let data = sqlx::query_as!(
            DbUser,
            r#"
                insert into federated_user (id, username, last_refreshed_at, private_key, public_key, inbox, followers, local, ap_id)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                on conflict (id)
                do update set
                    username = excluded.username,
                    last_refreshed_at = excluded.last_refreshed_at,
                    private_key = excluded.private_key,
                    public_key = excluded.public_key,
                    inbox = excluded.inbox,
                    followers = excluded.followers,
                    local = excluded.local,
                    ap_id = excluded.ap_id
                returning *
            "#,
            id,
            username,
            OffsetDateTime::now_utc(),
            None::<String>,
            json.public_key.public_key_pem,
            json.inbox.as_str(),
            &[],
            false,
            ap_id_str,
        )
        .fetch_one(db)
        .instrument(info_span!("db.upsert.user"))
        .await?;

        write_to_cache::<()>(cache_key, &data, cache).await?;

        let payload = LocalUser::try_from(data)?;
        Ok(payload)

    }
}

impl Actor for LocalUser {
    fn id(&self) -> Url {
        Url::from_str(&self.id).unwrap()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key
            .as_ref()
            .map(|value| value.expose_secret().to_string())
    }

    fn inbox(&self) -> Url {
        self.inbox.clone()
    }
}
