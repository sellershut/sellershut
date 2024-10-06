#![allow(missing_docs, missing_debug_implementations)]
// https://github.com/svix/svix-webhooks/blob/main/server/svix-server/src/redis/mod.rs
mod cluster;

pub mod key;

use bb8::{Pool, RunError};
use bb8_redis::RedisConnectionManager;
pub use cluster::RedisClusterConnectionManager;

use async_trait::async_trait;
use redis::{FromRedisValue, RedisError, RedisResult, ToRedisArgs};

use crate::{ServiceError, ServicesBuilder};

#[derive(Clone, Debug)]
pub enum RedisPool {
    Clustered(ClusteredRedisPool),
    NonClustered(NonClusteredRedisPool),
}

#[derive(Clone, Debug)]
pub struct ClusteredRedisPool {
    pool: Pool<RedisClusterConnectionManager>,
}

#[derive(Clone, Debug)]
pub struct NonClusteredRedisPool {
    pool: Pool<RedisConnectionManager>,
}

pub enum PooledConnection<'a> {
    Clustered(ClusteredPooledConnection<'a>),
    NonClustered(NonClusteredPooledConnection<'a>),
}

#[async_trait]
pub trait PooledConnectionLike {
    async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T>;
    async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T>;

    #[cfg(feature = "cache-write")]
    async fn del<K: ToRedisArgs + Send, T: FromRedisValue>(&mut self, key: K) -> RedisResult<T> {
        self.query_async(redis::Cmd::del(key)).await
    }

    async fn get<K: ToRedisArgs + Send, T: FromRedisValue>(&mut self, key: K) -> RedisResult<T> {
        let mut cmd = redis::cmd(if key.num_of_args() == 1 {
            "GET"
        } else {
            "MGET"
        });
        cmd.arg(key);
        self.query_async(cmd).await
    }

    #[cfg(feature = "cache-write")]
    async fn lpop<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        count: Option<core::num::NonZeroUsize>,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::lpop(key, count)).await
    }

    async fn lrange<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        start: isize,
        stop: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::lrange(key, start, stop)).await
    }

    #[cfg(feature = "cache-write")]
    async fn lrem<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        count: isize,
        value: V,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::lrem(key, count, value)).await
    }

    #[cfg(feature = "cache-write")]
    async fn pset_ex<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        value: V,
        milliseconds: u64,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::pset_ex(key, value, milliseconds))
            .await
    }

    #[cfg(feature = "cache-write")]
    async fn rpush<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        value: V,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::rpush(key, value)).await
    }

    #[cfg(feature = "cache-write")]
    async fn set<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        value: V,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::set(key, value)).await
    }

    #[cfg(feature = "cache-write")]
    async fn zadd<
        K: ToRedisArgs + Send,
        S: ToRedisArgs + Send,
        M: ToRedisArgs + Send,
        T: FromRedisValue,
    >(
        &mut self,
        key: K,
        member: M,
        score: S,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zadd(key, member, score)).await
    }

    #[cfg(feature = "cache-write")]
    async fn zadd_multiple<
        K: ToRedisArgs + Send,
        S: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
        T: FromRedisValue,
    >(
        &mut self,
        key: K,
        items: &'_ [(S, M)],
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zadd_multiple(key, items))
            .await
    }

    #[cfg(feature = "cache-write")]
    async fn zpopmin<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        count: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zpopmin(key, count)).await
    }

    #[cfg(feature = "cache-write")]
    async fn zrange_withscores<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        start: isize,
        stop: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zrange_withscores(key, start, stop))
            .await
    }

    #[cfg(feature = "cache-write")]
    async fn zrangebyscore_limit<
        K: ToRedisArgs + Send,
        M: ToRedisArgs + Send,
        MM: ToRedisArgs + Send,
        T: FromRedisValue,
    >(
        &mut self,
        key: K,
        min: M,
        max: MM,
        offset: isize,
        count: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zrangebyscore_limit(
            key, min, max, offset, count,
        ))
        .await
    }
}

#[async_trait]
impl<'a> PooledConnectionLike for PooledConnection<'a> {
    async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        match self {
            Self::Clustered(pooled_con) => pooled_con.query_async(cmd).await,
            Self::NonClustered(pooled_con) => pooled_con.query_async(cmd).await,
        }
    }

    async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        match self {
            Self::Clustered(pooled_con) => pooled_con.query_async_pipeline(pipe).await,
            Self::NonClustered(pooled_con) => pooled_con.query_async_pipeline(pipe).await,
        }
    }
}

pub struct NonClusteredPooledConnection<'a> {
    con: bb8::PooledConnection<'a, RedisConnectionManager>,
}

impl<'a> NonClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }

    pub async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        pipe.query_async(&mut *self.con).await
    }
}

pub struct ClusteredPooledConnection<'a> {
    con: bb8::PooledConnection<'a, RedisClusterConnectionManager>,
}

impl<'a> ClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }

    pub async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        pipe.query_async(&mut *self.con).await
    }
}

#[async_trait]
pub trait PoolLike {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>>;
}

#[async_trait]
impl PoolLike for RedisPool {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>> {
        match self {
            Self::Clustered(pool) => pool.get().await,
            Self::NonClustered(pool) => pool.get().await,
        }
    }
}

#[async_trait]
impl PoolLike for NonClusteredRedisPool {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>> {
        let con = self.pool.get().await?;
        let con = NonClusteredPooledConnection { con };
        Ok(PooledConnection::NonClustered(con))
    }
}

#[async_trait]
impl PoolLike for ClusteredRedisPool {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>> {
        let con = ClusteredPooledConnection {
            con: self.pool.get().await?,
        };
        Ok(PooledConnection::Clustered(con))
    }
}

async fn new_redis_pool_helper(
    config: &crate::config::cache::CacheConfig,
) -> Result<RedisPool, ServiceError> {
    if config.is_cluster {
        let mgr = RedisClusterConnectionManager::new(config.dsn.to_string())?;
        let pool = bb8::Pool::builder()
            .max_size(config.max_connections)
            .build(mgr)
            .await?;
        let pool = ClusteredRedisPool { pool };
        Ok(RedisPool::Clustered(pool))
    } else {
        let mgr = RedisConnectionManager::new(config.dsn.to_string())?;
        let pool = bb8::Pool::builder()
            .max_size(config.max_connections)
            .build(mgr)
            .await?;
        let pool = NonClusteredRedisPool { pool };
        Ok(RedisPool::NonClustered(pool))
    }
}

impl ServicesBuilder {
    #[cfg(feature = "cache")]
    pub async fn with_cache(
        mut self,
        config: &crate::config::cache::CacheConfig,
    ) -> Result<Self, crate::ServiceError> {
        log::trace!("initialising cache");

        self.cache = Some(new_redis_pool_helper(config).await?);

        Ok(self)
    }
}
