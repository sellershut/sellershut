pub mod error;
pub(crate) mod helpers;

use helpers::database::DatabaseActor;
use sellershut_core::{RedactedSecret, auth::OauthProvider, user::User};
use sellershut_svc::cache::Cache;
use sellershut_utilities::{auth::hash_token, cache_key::CacheKey};
use sqlx::PgConnection;
use std::time::Duration;
use tracing::{debug, info, trace};
use url::Url;
use uuid::Uuid;

use crate::error::UserError;

pub struct CreateUser {
    pub ap_id: Url,
    pub preferred_username: String,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub inbox: Url,
    pub outbox: Url,

    pub followers: Option<Url>,
    pub following: Option<Url>,

    pub likes: Option<Url>,

    pub kind: String,
    pub public_key: String,
    pub private_key: Option<RedactedSecret>,
    pub is_local: bool,
    pub icon: Option<Url>,
}

#[async_trait::async_trait]
pub trait UserDriver: Send + Sync {
    async fn get_user(&self, username: &str) -> Result<Option<User>, UserError>;
    async fn get_user_by_id(&self, ap_id: &Url) -> Result<Option<User>, UserError>;
    async fn get_system_user(&self, domain: &str) -> Result<Option<User>, UserError>;
    async fn create_user(
        &self,
        data: &CreateUser,
        tx: Option<&mut PgConnection>,
    ) -> Result<User, UserError>;
    async fn upsert_user(
        &self,
        data: &CreateUser,
        tx: Option<&mut PgConnection>,
    ) -> Result<User, UserError>;
    async fn user_from_session(&self, session_token: &str) -> Result<User, UserError>;

    async fn find_user_by_email(
        &self,
        email: &str,
        connection: Option<&mut sqlx::PgConnection>,
    ) -> Result<Option<User>, UserError>;

    async fn find_user_by_identity(
        &self,
        connection: Option<&mut sqlx::PgConnection>,
        provider: OauthProvider,
        provider_subject: &str,
    ) -> Result<Option<User>, UserError>;
}

pub struct UserService {
    database: sqlx::PgPool,
    cache: sellershut_svc::cache::Cache,
}

#[async_trait::async_trait]
impl UserDriver for UserService {
    async fn find_user_by_identity(
        &self,
        connection: Option<&mut sqlx::PgConnection>,
        provider: OauthProvider,
        provider_subject: &str,
    ) -> Result<Option<User>, UserError> {
        let q = sqlx::query_as!(
            DatabaseActor,
            r#"
            select
                u.*
            from oauth_identity as oi
            join actor as u on u.id = oi.user_id
            where oi.provider = $1
              and oi.provider_id = $2
            for update of oi
            "#,
            provider.to_string(),
            provider_subject
        );

        let user = match connection {
            Some(conn) => q.fetch_optional(conn).await,
            None => q.fetch_optional(&self.database).await,
        }?
        .map(User::from);

        Ok(user)
    }

    async fn find_user_by_email(
        &self,
        email: &str,
        connection: Option<&mut sqlx::PgConnection>,
    ) -> Result<Option<User>, UserError> {
        let q = sqlx::query_as!(
            DatabaseActor,
            r#"
                select u.* from actor as u
                join "oauth_identity" as oi on u.id = oi.user_id
                where
                    oi.provider_email = $1
                    and u.is_local
                for update
            "#,
            email
        );

        let user = match connection {
            Some(conn) => q.fetch_optional(conn).await,
            None => q.fetch_optional(&self.database).await,
        }?
        .map(User::from);

        Ok(user)
    }
    async fn get_user(&self, username: &str) -> Result<Option<User>, UserError> {
        trace!(username, "getting local user");

        let cache_key = CacheKey::LocalUserByUsername(username);

        if let Some(user) = self.get_cached_user(cache_key).await {
            return Ok(Some(user));
        }

        debug!(username, "loading local user from database");

        let result = sqlx::query_as!(
            DatabaseActor,
            r#"
            select * from actor
            where
                preferred_username = $1
                and is_local
        "#,
            username
        )
        .fetch_optional(&self.database)
        .await?
        .map(User::from);

        if let Some(user) = &result {
            trace!(
                user_id = %user.id,
                username,
                "local user loaded from database"
            );

            self.cache_user(user).await;
        } else {
            trace!(username, "local user not found");
        }

        Ok(result)
    }

    async fn get_system_user(&self, domain: &str) -> Result<Option<User>, UserError> {
        trace!(domain, "getting system user");
        let result = sqlx::query_as!(
            DatabaseActor,
            r#"
            select * from actor
            where
                ap_id = $1
                and is_local
        "#,
            domain
        )
        .fetch_optional(&self.database)
        .await?
        .map(User::from);

        trace!(found = result.is_some(), "system user lookup completed");

        Ok(result)
    }

    async fn create_user(
        &self,
        data: &CreateUser,
        tx: Option<&mut PgConnection>,
    ) -> Result<User, UserError> {
        trace!(
            username = %data.preferred_username,
            ap_id = %data.ap_id,
            local = data.is_local,
            "creating user"
        );

        let external_transaction = tx.is_some();

        let query = sqlx::query_as!(
            DatabaseActor,
            r#"
            insert into actor
            (
                id,
                ap_id,
                preferred_username,
                name,
                summary,
                inbox,
                outbox,
                following,
                followers,
                likes,
                icon,
                kind,
                is_local
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            on conflict do nothing
            returning *
            "#,
            Uuid::now_v7(),
            data.ap_id.as_str(),
            data.preferred_username,
            data.name,
            data.summary,
            data.inbox.to_string(),
            data.outbox.to_string(),
            data.followers.as_ref().map(|v| v.as_str()),
            data.following.as_ref().map(|v| v.as_str()),
            data.likes.as_ref().map(|v| v.as_str()),
            data.icon.as_ref().map(|v| v.as_str()),
            data.kind,
            data.is_local,
        );

        let result = match tx {
            Some(connection) => query.fetch_optional(connection).await,
            None => query.fetch_optional(&self.database).await,
        }?
        .map(User::from);

        let user = if let Some(user) = result {
            if user.is_local {
                info!(
                    user_id = %user.id,
                    username = %user.preferred_username,
                    "local user created"
                );
            } else {
                debug!(
                    user_id = %user.id,
                    ap_id = %user.ap_id,
                    "remote user created"
                );
            }

            // Removing an entry before commit is safe:
            self.invalidate_cache_key(CacheKey::UserByApId(&user.ap_id))
                .await;

            if user.is_local {
                self.invalidate_cache_key(CacheKey::LocalUserByUsername(&user.preferred_username))
                    .await;
            }

            if !external_transaction {
                self.cache_user(&user).await;
            } else {
                trace!(
                    user_id = %user.id,
                    "not populating cache before external transaction commits"
                );
            }

            user
        } else {
            debug!(
                username = %data.preferred_username,
                "user insert conflicted"
            );

            self.get_user(&data.preferred_username)
                .await?
                .ok_or(UserError::UsernameTaken)?
        };

        Ok(user)
    }

    async fn upsert_user(
        &self,
        data: &CreateUser,
        mut tx: Option<&mut PgConnection>,
    ) -> Result<User, UserError> {
        trace!(
            username = %data.preferred_username,
            ap_id = %data.ap_id,
            "upserting user"
        );

        let external_transaction = tx.is_some();

        let previous = {
            let query = sqlx::query!(
                r#"
            select
                preferred_username,
                is_local
            from actor
            where ap_id = $1
            "#,
                data.ap_id.as_str()
            );

            match tx.as_deref_mut() {
                Some(connection) => query.fetch_optional(connection).await?,

                None => query.fetch_optional(&self.database).await?,
            }
        };

        let query = sqlx::query_as!(
            DatabaseActor,
            r#"
            insert into actor
            (
                id,
                ap_id,
                preferred_username,
                name,
                summary,
                inbox,
                outbox,
                following,
                followers,
                likes,
                icon,
                kind,
                is_local
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            on conflict (ap_id) do update set
                preferred_username = excluded.preferred_username,
                    name = excluded.name,
                    summary = excluded.summary,
                    inbox = excluded.inbox,
                    outbox = excluded.outbox,
                    following = excluded.following,
                    followers = excluded.followers,
                    likes = excluded.likes,
                    icon = excluded.icon,
                    kind = excluded.kind,
                    is_local = excluded.is_local
            returning *
            "#,
            Uuid::now_v7(),
            data.ap_id.as_str(),
            data.preferred_username,
            data.name,
            data.summary,
            data.inbox.to_string(),
            data.outbox.to_string(),
            data.followers.as_ref().map(|v| v.as_str()),
            data.following.as_ref().map(|v| v.as_str()),
            data.likes.as_ref().map(|v| v.as_str()),
            data.icon.as_ref().map(|v| v.as_str()),
            data.kind,
            data.is_local,
        );

        let user = match tx {
            Some(connection) => query.fetch_one(connection).await?,

            None => query.fetch_one(&self.database).await?,
        };
        let user = User::from(user);

        debug!(
            user_id = %user.id,
            ap_id = %user.ap_id,
            "user upserted"
        );

        self.invalidate_cache_key(CacheKey::UserByApId(&user.ap_id))
            .await;

        // Remove the previous local username if it existed.
        if let Some(previous) = previous
            && previous.is_local
        {
            self.invalidate_cache_key(CacheKey::LocalUserByUsername(&previous.preferred_username))
                .await;
        }

        // Also remove the new username key in case it already existed.
        if user.is_local {
            self.invalidate_cache_key(CacheKey::LocalUserByUsername(&user.preferred_username))
                .await;
        }

        // Safe only when the operation has already committed.
        if !external_transaction {
            self.cache_user(&user).await;
        } else {
            trace!(
                user_id = %user.id,
                "not populating cache before external transaction commits"
            );
        }

        Ok(user)
    }

    async fn user_from_session(&self, session_token: &str) -> Result<User, UserError> {
        trace!("resolving user from session");

        let user = sqlx::query_as!(
            DatabaseActor,
            r#"
            select u.*
            from auth_session as s
            join actor as u on u.id = s.user_id
            where s.token_hash = $1
              and s.expires_at > now()
            "#,
            hash_token(session_token)
        )
        .fetch_one(&self.database)
        .await?;

        let user = User::from(user);

        trace!(
            user_id = %user.id,
            "session resolved"
        );

        Ok(user)
    }

    async fn get_user_by_id(&self, ap_id: &Url) -> Result<Option<User>, UserError> {
        trace!(
            ap_id = %ap_id,
            "getting user by ActivityPub id"
        );

        let cache_key = CacheKey::UserByApId(&ap_id.into());

        if let Some(user) = self.get_cached_user(cache_key).await {
            return Ok(Some(user));
        }

        debug!(
            ap_id = %ap_id,
            "loading user from database"
        );

        let result = sqlx::query_as!(
            DatabaseActor,
            r#"
            select *
            from actor
            where
                ap_id = $1
        "#,
            ap_id.as_str()
        )
        .fetch_optional(&self.database)
        .await?
        .map(User::from);

        if let Some(user) = &result {
            trace!(
                user_id = %user.id,
                ap_id = %ap_id,
                "user loaded from database"
            );

            self.cache_user(user).await;
        } else {
            trace!(
                ap_id = %ap_id,
                "user not found"
            );
        }

        Ok(result)
    }
}

const USER_CACHE_TTL: Duration = Duration::from_secs(5 * 60);

impl UserService {
    pub fn new(pool: sqlx::PgPool, cache: Cache) -> Self {
        Self {
            database: pool,
            cache,
        }
    }

    async fn get_cached_user(&self, key: CacheKey<'_>) -> Option<User> {
        trace!(
            cache_key = %key,
            "checking user cache"
        );

        let cached = match self.cache.get::<Vec<u8>>(key).await {
            Ok(Some(value)) => value,
            Ok(None) => {
                debug!(
                    cache_key = %key,
                    "user cache miss"
                );
                return None;
            }
            Err(error) => {
                debug!(
                    cache_key = %key,
                    error = %error,
                    "cache read failed; falling back to database"
                );
                return None;
            }
        };

        match serde_json::from_slice::<User>(&cached) {
            Ok(user) => {
                trace!(
                    cache_key = %key,
                    user_id = %user.id,
                    "user cache hit"
                );
                Some(user)
            }

            Err(error) => {
                debug!(
                    cache_key = %key,
                    error = %error,
                    "cached user could not be deserialized; evicting entry"
                );

                if let Err(error) = self.cache.del(key).await {
                    debug!(
                        cache_key = %key,
                        error = %error,
                        "failed to evict invalid cache entry"
                    );
                }

                None
            }
        }
    }

    async fn cache_user(&self, user: &User) {
        let value = match serde_json::to_vec(user) {
            Ok(value) => value,
            Err(error) => {
                debug!(
                    user_id = %user.id,
                    error = %error,
                    "failed to serialize user for cache"
                );

                return;
            }
        };

        let ap_id_key = CacheKey::UserByApId(&user.ap_id);

        self.cache_user_key(ap_id_key, &value).await;

        // cache them by username if ttey are local
        if user.is_local {
            let username_key = CacheKey::LocalUserByUsername(&user.preferred_username);
            self.cache_user_key(username_key, &value).await;
        }
    }

    async fn cache_user_key(&self, key: CacheKey<'_>, value: &[u8]) {
        trace!(
            cache_key = %key,
            "populating user cache"
        );

        if let Err(error) = self.cache.set_ex(key, value, USER_CACHE_TTL).await {
            debug!(
                cache_key = %key,
                error = %error,
                "failed to populate user cache"
            );
        }
    }

    async fn invalidate_cache_key(&self, key: CacheKey<'_>) {
        trace!(
            cache_key = %key,
            "invalidating user cache"
        );

        if let Err(error) = self.cache.del(key).await {
            debug!(
                cache_key = %key,
                error = %error,
                "failed to invalidate user cache"
            );
        }
    }
}
