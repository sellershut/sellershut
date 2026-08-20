use sellershut_core::RedactedSecret;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(flatten)]
    pub database: DatabaseConfig,
    pub max_connections: MaxConnections,
}

impl Config {
    pub async fn connect(&self) -> Result<sqlx::PgPool, sqlx::Error> {
        let database = match &self.database {
            DatabaseConfig::Url { url } => sqlx::PgPool::connect(url.as_str()).await,

            DatabaseConfig::Connection {
                username,
                password,
                host,
                db_name,
            } => {
                sqlx::postgres::PgPoolOptions::new()
                    .connect_with(
                        sqlx::postgres::PgConnectOptions::new()
                            .host(host)
                            .username(username)
                            .password(&password.expose())
                            .database(db_name),
                    )
                    .await
            }
        }?;
        sqlx::migrate!("../../migrations").run(&database).await?;

        Ok(database)
    }
}

/// Max database connections
#[derive(Deserialize, Debug, Serialize)]
pub struct MaxConnections(u32);
impl Default for MaxConnections {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DatabaseConfig {
    Url {
        url: Url,
    },
    Connection {
        username: String,
        password: RedactedSecret,
        host: String,
        db_name: String,
    },
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::Url {
            url: Url::parse("postgres://postgres:password@localhost:5432/sellershut")
                .expect("valid url"),
        }
    }
}

#[cfg(test)]
mod tests;
