use redis::{ToRedisArgs, ToSingleRedisArg};

#[derive(Clone, Copy)]
pub enum CacheKey<'a> {
    Session(&'a str),
}

impl ToRedisArgs for CacheKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(
            match self {
                CacheKey::Session(id) => {
                    format!("session:{id}")
                }
            }
            .as_bytes(),
        );
    }
}

impl ToSingleRedisArg for CacheKey<'_> {}
