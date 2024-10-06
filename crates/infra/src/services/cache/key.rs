use std::fmt::Display;

use redis::ToRedisArgs;

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum CacheKey<'a> {
    Categories(CursorParams<'a>),
    CategoriesSubCategory(CursorParams<'a>),
    Category(&'a str),
}

#[derive(Clone, Copy, Debug)]
pub struct CursorParams<'a> {
    pub cursor: Option<&'a str>,
    pub index: Index,
}

impl Display for CursorParams<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cursor={}:index={}",
            self.cursor.unwrap_or("[NONE]"),
            match self.index {
                Index::First(v) => format!("first:{v}"),
                Index::Last(v) => format!("last:{v}"),
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Index {
    First(i32),
    Last(i32),
}

impl Display for CacheKey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CacheKey::Categories(params) => format!("categories:all:{params}"),
                CacheKey::CategoriesSubCategory(params) =>
                    format!("categories:subcategories:{params}"),
                CacheKey::Category(id) => format!("categories:id={id}"),
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
