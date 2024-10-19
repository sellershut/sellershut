use async_session::{async_trait, serde_json, Result, Session, SessionStore};
use sqlx::PgPool;
use time::OffsetDateTime;
use tracing::error;

#[derive(Clone, Debug)]
pub struct PostgresSessionStore {
    client: PgPool,
    table_name: String,
}

impl PostgresSessionStore {
    pub fn from_client(client: PgPool) -> Self {
        Self {
            client,
            table_name: "session".into(),
        }
    }

    pub async fn new(database_url: &str) -> sqlx::Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self::from_client(pool))
    }
    pub async fn new_with_table_name(database_url: &str, table_name: &str) -> sqlx::Result<Self> {
        Ok(Self::new(database_url).await?.with_table_name(table_name))
    }

    pub fn with_table_name(mut self, table_name: impl AsRef<str>) -> Self {
        let table_name = table_name.as_ref();
        if table_name.is_empty()
            || !table_name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            panic!(
                "table name must be [a-zA-Z0-9_-]+, but {} was not",
                table_name
            );
        }

        self.table_name = table_name.to_owned();
        self
    }

    fn substitute_table_name(&self, query: &str) -> String {
        query.replace("%%TABLE_NAME%%", &self.table_name)
    }

    pub async fn cleanup(&self) -> sqlx::Result<()> {
        sqlx::query(&self.substitute_table_name("DELETE FROM %%TABLE_NAME%% WHERE expires_at < $1"))
            .bind(OffsetDateTime::now_utc())
            .execute(&self.client)
            .await?;

        Ok(())
    }

    pub async fn count(&self) -> sqlx::Result<i64> {
        let (count,) =
            sqlx::query_as(&self.substitute_table_name("SELECT COUNT(*) FROM %%TABLE_NAME%%"))
                .fetch_one(&self.client)
                .await?;

        Ok(count)
    }

    pub fn spawn_cleanup_task(&self, period: std::time::Duration) -> tokio::task::JoinHandle<()> {
        let store = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(period).await;
                if let Err(error) = store.cleanup().await {
                    error!("cleanup error: {}", error);
                }
            }
        })
    }
}

#[async_trait]
impl SessionStore for PostgresSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;

        let result: Option<(String,)> = sqlx::query_as(&self.substitute_table_name(
            "SELECT session FROM %%TABLE_NAME%% WHERE id = $1 AND (expires_at IS NULL OR expires_at > $2)"
        ))
        .bind(&id)
        .bind(OffsetDateTime::now_utc())
        .fetch_optional(&self.client)
        .await?;

        Ok(result
            .map(|(session,)| serde_json::from_str(&session))
            .transpose()?)
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let string = serde_json::to_string(&session)?;
        let expiry = session
            .expiry()
            .map(|f| OffsetDateTime::from_unix_timestamp(f.timestamp_millis()).unwrap());

        sqlx::query(&self.substitute_table_name(
            r#"
            INSERT INTO %%TABLE_NAME%%
              (id, session, expires_at) SELECT $1, $2, $3
            ON CONFLICT(id) DO UPDATE SET
              expires_at = EXCLUDED.expires_at,
              session = EXCLUDED.session
            "#,
        ))
        .bind(&id)
        .bind(&string)
        .bind(&expiry)
        .execute(&self.client)
        .await?;

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let id = session.id();
        sqlx::query(&self.substitute_table_name("DELETE FROM %%TABLE_NAME%% WHERE id = $1"))
            .bind(&id)
            .execute(&self.client)
            .await?;

        Ok(())
    }

    async fn clear_store(&self) -> Result {
        sqlx::query(&self.substitute_table_name("TRUNCATE %%TABLE_NAME%%"))
            .execute(&self.client)
            .await?;

        Ok(())
    }
}
