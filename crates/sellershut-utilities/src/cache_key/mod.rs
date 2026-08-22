use redis::{RedisWrite, ToRedisArgs, ToSingleRedisArg};
use sellershut_core::Url;
use std::fmt;

const CACHE_NAMESPACE: &str = "app:v1";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CacheKey<'a> {
    LocalUserByUsername(&'a str),
    UserByApId(&'a Url),
}

impl CacheKey<'_> {
    fn redis_key(&self) -> String {
        match self {
            Self::LocalUserByUsername(username) => {
                format!("{CACHE_NAMESPACE}:user:local:username:{username}")
            }

            Self::UserByApId(ap_id) => {
                format!("{CACHE_NAMESPACE}:user:ap-id:{ap_id}")
            }
        }
    }
}

impl fmt::Display for CacheKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.redis_key())
    }
}

impl ToRedisArgs for CacheKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let key = self.redis_key();
        out.write_arg(key.as_bytes());
    }
}

impl ToSingleRedisArg for CacheKey<'_> {}
