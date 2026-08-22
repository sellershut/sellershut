pub mod error;

use sellershut_core::{
    RedactedSecret,
    user::{ActorType, User},
};
use sellershut_svc::cache::Cache;
use sellershut_utilities::{auth::hash_token, cache_key::CacheKey};
use sqlx::PgConnection;
use std::time::Duration;
use tracing::{debug, info, trace};
use url::Url;
use uuid::Uuid;

use crate::error::UserError;

pub struct CreateUser {
    pub kind: ActorType,
    pub username: String,
    pub ap_id: Url,
    pub name: Option<String>,
    pub inbox: Url,
    pub public_key: String,
    pub private_key: Option<RedactedSecret>,
    pub is_local: bool,
    pub avatar: Option<Url>,
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
}

pub struct UserService {
    database: sqlx::PgPool,
    cache: sellershut_svc::cache::Cache,
}

#[async_trait::async_trait]
impl UserDriver for UserService {
    async fn get_user(&self, username: &str) -> Result<Option<User>, UserError> {
        trace!(username, "getting local user");

        let cache_key = CacheKey::LocalUserByUsername(username);

        if let Some(user) = self.get_cached_user(cache_key).await {
            return Ok(Some(user));
        }

        debug!(username, "loading local user from database");

        let result = sqlx::query_as!(
            User,
            r#"
            select
                id,
                ap_id,
                username,
                name,
                inbox,
                public_key,
                private_key as "private_key: RedactedSecret",
                kind as "kind: ActorType",
                last_refreshed_at,
                created_at,
                avatar,
                is_local
            from "user"
            where
                username = $1
                and is_local
        "#,
            username
        )
        .fetch_optional(&self.database)
        .await?;

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
            User,
            r#"
            select
                id,
                ap_id,
                username,
                name,
                inbox,
                public_key,
                avatar,
                private_key as "private_key: RedactedSecret",
                kind as "kind: ActorType",
                last_refreshed_at,
                created_at,
                is_local
            from "user"
            where
                ap_id = $1
                and is_local
        "#,
            domain
        )
        .fetch_optional(&self.database)
        .await?;

        trace!(found = result.is_some(), "system user lookup completed");

        Ok(result)
    }

    async fn create_user(
        &self,
        data: &CreateUser,
        tx: Option<&mut PgConnection>,
    ) -> Result<User, UserError> {
        trace!(
            username = %data.username,
            ap_id = %data.ap_id,
            local = data.is_local,
            "creating user"
        );

        let external_transaction = tx.is_some();

        let query = sqlx::query_as!(
            User,
            r#"
            insert into "user"
            (
                id,
                ap_id,
                username,
                name,
                inbox,
                public_key,
                avatar,
                private_key,
                kind,
                is_local
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            on conflict do nothing
            returning
                id,
                ap_id,
                username,
                name,
                inbox,
                public_key,
                avatar,
                private_key as "private_key: RedactedSecret",
                kind as "kind: ActorType",
                last_refreshed_at,
                created_at,
                is_local
            "#,
            Uuid::now_v7(),
            data.ap_id.as_str(),
            data.username,
            data.name,
            data.inbox.to_string(),
            data.public_key,
            data.avatar.as_ref().map(|v| v.as_str()),
            data.private_key as _,
            data.kind as _,
            data.is_local,
        );

        let result = match tx {
            Some(connection) => query.fetch_optional(connection).await,
            None => query.fetch_optional(&self.database).await,
        }?;

        let user = if let Some(user) = result {
            if user.is_local {
                info!(
                    user_id = %user.id,
                    username = %user.username,
                    "local user created"
                );
            } else {
                debug!(
                    user_id = %user.id,
                    ap_id = ?user.ap_id,
                    "remote user created"
                );
            }

            // Removing an entry before commit is safe:
            let k = user.ap_id.inner();
            self.invalidate_cache_key(CacheKey::UserByApId(k.as_str()))
                .await;

            if user.is_local {
                self.invalidate_cache_key(CacheKey::LocalUserByUsername(&user.username))
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
                username = %data.username,
                "user insert conflicted"
            );

            self.get_user(&data.username)
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
            username = %data.username,
            ap_id = %data.ap_id,
            "upserting user"
        );

        let external_transaction = tx.is_some();

        let previous = {
            let query = sqlx::query!(
                r#"
            select
                username,
                is_local
            from "user"
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
            User,
            r#"
        insert into "user"
        (
            id,
            ap_id,
            username,
            name,
            inbox,
            public_key,
            avatar,
            private_key,
            kind,
            is_local
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        on conflict (ap_id) do update set
            username = excluded.username,
            name = excluded.name,
            inbox = excluded.inbox,
            public_key = excluded.public_key,
            avatar = excluded.avatar,
            private_key = excluded.private_key,
            kind = excluded.kind,
            is_local = excluded.is_local
        returning
            id,
            ap_id,
            username,
            name,
            inbox,
            public_key,
            avatar,
            private_key as "private_key: RedactedSecret",
            kind as "kind: ActorType",
            last_refreshed_at,
            created_at,
            is_local
        "#,
            Uuid::now_v7(),
            data.ap_id.as_str(),
            data.username,
            data.name,
            data.inbox.to_string(),
            data.public_key,
            data.avatar.as_ref().map(|v| v.as_str()),
            data.private_key as _,
            data.kind as _,
            data.is_local,
        );

        let user = match tx {
            Some(connection) => query.fetch_one(connection).await?,

            None => query.fetch_one(&self.database).await?,
        };

        debug!(
            user_id = %user.id,
            ap_id = %user.ap_id.inner(),
            "user upserted"
        );

        // Canonical AP id entry.
        let k = user.ap_id.inner();
        self.invalidate_cache_key(CacheKey::UserByApId(k.as_str()))
            .await;

        // Remove the previous local username if it existed.
        if let Some(previous) = previous
            && previous.is_local
        {
            self.invalidate_cache_key(CacheKey::LocalUserByUsername(&previous.username))
                .await;
        }

        // Also remove the new username key in case it already existed.
        if user.is_local {
            self.invalidate_cache_key(CacheKey::LocalUserByUsername(&user.username))
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
            User,
            r#"
            SELECT 
                u.id,
                u.ap_id,
                u.username,
                u.name,
                u.inbox,
                u.public_key,
                u.avatar,
                u.private_key as "private_key: RedactedSecret",
                u.kind as "kind: ActorType",
                u.last_refreshed_at,
                u.created_at,
                u.is_local
            FROM auth_session AS s
            JOIN "user" AS u ON u.id = s.user_id
            WHERE s.token_hash = $1
              AND s.expires_at > now()
            "#,
            hash_token(session_token)
        )
        .fetch_one(&self.database)
        .await?;

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

        let cache_key = CacheKey::UserByApId(ap_id.as_str());

        if let Some(user) = self.get_cached_user(cache_key).await {
            return Ok(Some(user));
        }

        debug!(
            ap_id = %ap_id,
            "loading user from database"
        );

        let result = sqlx::query_as!(
            User,
            r#"
            select
                id,
                ap_id,
                username,
                name,
                inbox,
                public_key,
                avatar,
                private_key as "private_key: RedactedSecret",
                kind as "kind: ActorType",
                last_refreshed_at,
                created_at,
                is_local
            from "user"
            where
                ap_id = $1
        "#,
            ap_id.as_str()
        )
        .fetch_optional(&self.database)
        .await?;

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

        let cached = match self.cache.get::<String>(key).await {
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

        match serde_json::from_str::<User>(&cached) {
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
        let value = match serde_json::to_string(user) {
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

        let k = user.ap_id.inner();
        let ap_id_key = CacheKey::UserByApId(k.as_str());

        self.cache_user_key(ap_id_key, &value).await;

        if user.is_local {
            let username_key = CacheKey::LocalUserByUsername(&user.username);

            self.cache_user_key(username_key, &value).await;
        }
    }

    async fn cache_user_key(&self, key: CacheKey<'_>, value: &str) {
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
