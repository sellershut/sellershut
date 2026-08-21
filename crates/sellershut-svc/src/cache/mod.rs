use redis::{
    AsyncCommands,
    aio::MultiplexedConnection,
    cluster::ClusterClient,
    sentinel::{SentinelClient, SentinelServerType},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("failed to read config: {0}")]
    ConfigRead(#[from] std::io::Error),
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mode: Mode,

    pub standalone: Option<StandaloneConfig>,
    pub sentinel: Option<SentinelConfig>,
    pub cluster: Option<ClusterConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Standalone,
    Sentinel,
    Cluster,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandaloneConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SentinelConfig {
    pub service_name: String,
    pub sentinels: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClusterConfig {
    pub nodes: Vec<String>,
}

#[derive(Clone)]
enum Connection {
    Standalone(MultiplexedConnection),
    Sentinel(MultiplexedConnection),
    Cluster(redis::cluster_async::ClusterConnection),
}

#[derive(Clone)]
pub struct Cache {
    connection: Connection,
}

impl Cache {
    pub async fn connect(config: &Config) -> Result<Self, CacheError> {
        let connection = match config.mode {
            Mode::Standalone => {
                let cfg = config.standalone.as_ref().ok_or_else(|| {
                    CacheError::InvalidConfig(
                        "mode is standalone but [standalone] is missing".to_string(),
                    )
                })?;

                let client = redis::Client::open(cfg.url.as_str())?;

                let connection = client.get_multiplexed_async_connection().await?;

                Connection::Standalone(connection)
            }

            Mode::Sentinel => {
                let cfg = config.sentinel.as_ref().ok_or_else(|| {
                    CacheError::InvalidConfig(
                        "mode is sentinel but [sentinel] is missing".to_string(),
                    )
                })?;

                if cfg.sentinels.is_empty() {
                    return Err(CacheError::InvalidConfig(
                        "at least one Sentinel node is required".to_string(),
                    ));
                }

                let nodes = cfg.sentinels.iter().map(String::as_str).collect();

                let mut sentinel = SentinelClient::build(
                    nodes,
                    cfg.service_name.to_string(),
                    None,
                    SentinelServerType::Master,
                )?;

                let connection = sentinel.get_async_connection().await?;

                Connection::Sentinel(connection)
            }

            Mode::Cluster => {
                let cfg = config.cluster.as_ref().ok_or_else(|| {
                    CacheError::InvalidConfig(
                        "mode is cluster but [cluster] is missing".to_string(),
                    )
                })?;

                if cfg.nodes.is_empty() {
                    return Err(CacheError::InvalidConfig(
                        "at least one cluster node is required".to_string(),
                    ));
                }

                let client = ClusterClient::new(cfg.nodes.clone())?;

                let connection = client.get_async_connection().await?;

                Connection::Cluster(connection)
            }
        };

        Ok(Self { connection })
    }

    pub async fn set(
        &self,
        key: impl redis::ToSingleRedisArg + Send + Sync,
        value: impl redis::ToSingleRedisArg + Send + Sync,
    ) -> Result<(), CacheError> {
        let mut connection = self.connection.clone();
        match connection {
            Connection::Standalone(ref mut connection)
            | Connection::Sentinel(ref mut connection) => {
                connection.set::<_, _, ()>(key, value).await?;
            }

            Connection::Cluster(ref mut connection) => {
                connection.set::<_, _, ()>(key, value).await?;
            }
        }

        Ok(())
    }

    pub async fn get<T>(
        &self,
        key: impl redis::ToSingleRedisArg + Send + Sync,
    ) -> Result<Option<T>, CacheError>
    where
        T: redis::FromRedisValue,
    {
        let mut connection = self.connection.clone();
        let value = match connection {
            Connection::Standalone(ref mut connection)
            | Connection::Sentinel(ref mut connection) => connection.get(key).await?,

            Connection::Cluster(ref mut connection) => connection.get(key).await?,
        };

        Ok(value)
    }

    pub async fn del(
        &self,
        key: impl redis::ToSingleRedisArg + Send + Sync,
    ) -> Result<(), CacheError> {
        let mut connection = self.connection.clone();
        match connection {
            Connection::Standalone(ref mut connection)
            | Connection::Sentinel(ref mut connection) => {
                connection.del::<_, ()>(key).await?;
            }

            Connection::Cluster(ref mut connection) => {
                connection.del::<_, ()>(key).await?;
            }
        }

        Ok(())
    }

    pub async fn ping(&self) -> Result<(), CacheError> {
        let mut connection = self.connection.clone();
        match connection {
            Connection::Standalone(ref mut connection)
            | Connection::Sentinel(ref mut connection) => {
                redis::cmd("PING").query_async::<String>(connection).await?;
            }

            Connection::Cluster(ref mut connection) => {
                redis::cmd("PING").query_async::<String>(connection).await?;
            }
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Standalone,
            standalone: Some(StandaloneConfig {
                url: "redis://127.0.0.1:6379".to_string(),
            }),

            sentinel: Some(SentinelConfig {
                service_name: "mymaster".to_string(),
                sentinels: vec![
                    "redis://127.0.0.1:26379".to_string(),
                    "redis://127.0.0.1:26380".to_string(),
                    "redis://127.0.0.1:26381".to_string(),
                ],
            }),
            cluster: Some(ClusterConfig {
                nodes: vec![
                    "redis://127.0.0.1:7000".to_string(),
                    "redis://127.0.0.1:7001".to_string(),
                    "redis://127.0.0.1:7002".to_string(),
                ],
            }),
        }
    }
}
