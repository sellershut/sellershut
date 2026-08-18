pub mod error;

use sellershut_core::types::{
    RedactedSecret,
    user::{ActorType, User},
};
use url::Url;
use uuid::Uuid;

use crate::error::UserError;

pub struct CreateUser {
    pub kind: ActorType,
    pub username: String,
    pub name: Option<String>,
    pub inbox: Url,
    pub public_key: String,
    pub private_key: Option<String>,
    pub is_local: bool,
}

#[async_trait::async_trait]
pub trait UserDriver: Send + Sync {
    async fn get_user(&self, username: &str) -> Result<Option<User>, UserError>;
    async fn create_user(&self, data: &CreateUser) -> Result<User, UserError>;
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
                username,
                name,
                inbox,
                public_key,
                private_key as "private_key: RedactedSecret",
                kind as "kind: ActorType",
                last_refreshed_at,
                created_at,
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

    async fn create_user(&self, data: &CreateUser) -> Result<User, UserError> {
        let user = sqlx::query_as!(
            User,
            r#"
            insert into "user"
            (
                id,
                username,
                name,
                inbox,
                public_key,
                private_key,
                kind,
                is_local
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8)
            returning 
                id,
                username,
                name,
                inbox,
                public_key,
                private_key as "private_key: RedactedSecret",
                kind as "kind: ActorType",
                last_refreshed_at,
                created_at,
                is_local
        "#,
            Uuid::now_v7(),
            data.username,
            data.name,
            data.inbox.to_string(),
            data.public_key,
            data.private_key as _,
            data.kind as _,
            data.is_local,
        )
        .fetch_one(&self.database)
        .await?;

        Ok(user)
    }
}

impl UserService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { database: pool }
    }
}
