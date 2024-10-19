use infra::services::cache::{PooledConnection, PooledConnectionLike};
use serde::Serialize;
use tracing::{info_span, Instrument};

use crate::state::cache::CacheKey;

pub mod category;
//pub mod listing;
pub mod user;

pub async fn write_to_cache<T: redis::FromRedisValue>(
    cache_key: CacheKey<'_>,
    data: impl Serialize,
    mut cache: PooledConnection<'_>,
) -> anyhow::Result<T> {
    let encoded: Vec<u8> = bincode::serialize(&data)?;
    Ok(cache
        .set::<_, _, T>(cache_key, &encoded)
        .instrument(info_span!("cache.set"))
        .await?)
}
