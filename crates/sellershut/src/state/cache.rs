use std::fmt::Display;

use redis::ToRedisArgs;
use url::Url;

#[derive(Debug, Clone, Copy)]
pub enum CacheKey<'a> {
    UserById(&'a Url),
    UserByName(&'a str),
}

impl Display for CacheKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "federated:{}",
            match self {
                CacheKey::UserById(id) => format!("user:id:{id}"),
                CacheKey::UserByName(name) => format!("user:name:{name}"),
            }
        )
    }
}

impl ToRedisArgs for CacheKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.to_string().as_bytes())
    }
}
