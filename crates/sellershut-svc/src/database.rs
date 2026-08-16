use sellershut_core::types::RedactedSecret;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug,Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(flatten)]
    pub database: DatabaseConfig,
    pub max_connections: MaxConnections,
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
