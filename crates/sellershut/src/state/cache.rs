use redis::aio::MultiplexedConnection;
use redis::cluster_async::ClusterConnection;

pub enum RedisConnection {
    Standalone(MultiplexedConnection),
    Cluster(ClusterConnection),
}

impl RedisClient {
    pub async fn get_connection(&self) -> redis::RedisResult<RedisConnection> {
        match self {
            RedisClient::Standalone(client) => Ok(RedisConnection::Standalone(
                client.get_multiplexed_async_connection().await?,
            )),
            RedisClient::Cluster(client) => Ok(RedisConnection::Cluster(
                client.get_async_connection().await?,
            )),
        }
    }
}

use redis::cluster::ClusterClient;
use redis::{AsyncCommands, Client, RedisResult, ToSingleRedisArg};
use tracing::{debug, info, trace};

use crate::config::cache::RedisConfig;
use crate::server::cache_key::CacheKey;

#[derive(Clone)]
pub enum RedisClient {
    Standalone(Client),
    Cluster(ClusterClient),
}

impl RedisClient {
    pub async fn new(config: &RedisConfig) -> RedisResult<Self> {
        let client = match config {
            RedisConfig::Standalone { url } => {
                debug!("Connecting to Redis standalone instance");
                trace!(redis_url = %url, "Creating Redis client");

                Self::Standalone(Client::open(url.as_str())?)
            }

            RedisConfig::Cluster { nodes } => {
                debug!(node_count = nodes.len(), "Connecting to Redis cluster");
                trace!(?nodes, "Creating Redis cluster client");

                Self::Cluster(ClusterClient::new(nodes.clone())?)
            }
        };

        client.ping().await?;

        info!("Redis connectivity check succeeded");

        Ok(client)
    }

    async fn ping(&self) -> RedisResult<()> {
        debug!("Performing Redis PING");

        match self {
            RedisClient::Standalone(client) => {
                let mut conn = client.get_multiplexed_async_connection().await?;
                let response: String = redis::cmd("PING").query_async(&mut conn).await?;

                debug!(response = %response, "Received Redis PING response");
            }

            RedisClient::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                let response: String = redis::cmd("PING").query_async(&mut conn).await?;

                debug!(response = %response, "Received Redis cluster PING response");
            }
        }

        Ok(())
    }

    pub async fn get<T>(&self, key: CacheKey<'_>) -> RedisResult<T>
    where
        T: redis::FromRedisValue,
    {
        match &self {
            RedisClient::Standalone(client) => {
                let mut conn = client.get_multiplexed_async_connection().await?;
                conn.get(key).await
            }

            RedisClient::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                conn.get(key).await
            }
        }
    }

    pub async fn set<T>(&self, key: CacheKey<'_>, value: T) -> RedisResult<()>
    where
        T: redis::ToRedisArgs + Send + Sync + ToSingleRedisArg,
    {
        match &self {
            RedisClient::Standalone(client) => {
                let mut conn = client.get_multiplexed_async_connection().await?;
                conn.set(key, value).await
            }

            RedisClient::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                conn.set(key, value).await
            }
        }
    }

    pub async fn set_ex<T>(&self, key: CacheKey<'_>, value: T, ttl_secs: u64) -> RedisResult<()>
    where
        T: redis::ToRedisArgs + Send + Sync + ToSingleRedisArg,
    {
        match &self {
            RedisClient::Standalone(client) => {
                let mut conn = client.get_multiplexed_async_connection().await?;
                conn.set_ex(key, value, ttl_secs).await
            }

            RedisClient::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                conn.set_ex(key, value, ttl_secs).await
            }
        }
    }

    pub async fn del(&self, key: CacheKey<'_>) -> RedisResult<()> {
        match &self {
            RedisClient::Standalone(client) => {
                let mut conn = client.get_multiplexed_async_connection().await?;
                conn.del(key).await
            }

            RedisClient::Cluster(client) => {
                let mut conn = client.get_async_connection().await?;
                conn.del(key).await
            }
        }
    }
}
