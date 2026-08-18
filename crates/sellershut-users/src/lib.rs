pub mod error;

use sellershut_core::types::user::{ActorType, User};

use crate::error::UserError;

#[async_trait::async_trait]
pub trait UserDriver: Send + Sync {
    async fn get_user(&self, username: &str) -> Result<Option<User>, UserError>;
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
                private_key,
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
}

impl UserService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { database: pool }
    }
}
