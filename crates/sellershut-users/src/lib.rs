pub mod error;

use sellershut_core::{
    RedactedSecret,
    user::{ActorType, User},
};
use sellershut_utilities::auth::hash_token;
use sqlx::PgConnection;
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
    async fn user_from_session(&self, session_token: &str) -> Result<User, UserError>;
}

pub struct UserService {
    database: sqlx::PgPool,
}

#[async_trait::async_trait]
impl UserDriver for UserService {
    async fn get_user(&self, username: &str) -> Result<Option<User>, UserError> {
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
        Ok(result)
    }
    async fn get_system_user(&self, domain: &str) -> Result<Option<User>, UserError> {
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

        Ok(result)
    }

    async fn create_user(
        &self,
        data: &CreateUser,
        tx: Option<&mut PgConnection>,
    ) -> Result<User, UserError> {
        let user = sqlx::query_as!(
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
            Some(a) => user.fetch_optional(a).await,
            None => user.fetch_optional(&self.database).await,
        }?;

        let user = if let Some(user) = result {
            user
        } else {
            // on conflicr results
            self.get_user(&data.username)
                .await?
                .ok_or(UserError::UsernameTaken)?
        };

        Ok(user)
    }

    async fn user_from_session(&self, session_token: &str) -> Result<User, UserError> {
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

        Ok(user)
    }

    async fn get_user_by_id(&self, ap_id: &Url) -> Result<Option<User>, UserError> {
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
        Ok(result)
    }
}

impl UserService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { database: pool }
    }
}
