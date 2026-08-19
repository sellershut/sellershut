#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Url(url::Url);
impl Url {
    pub fn inner(&self) -> url::Url {
        self.0.clone()
    }
}

impl Type<Postgres> for Url {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <String as Type<Postgres>>::compatible(ty)
    }
}

impl<'q> Encode<'q, Postgres> for Url {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        <String as Encode<Postgres>>::encode_by_ref(&self.0.as_str().to_owned(), buf)
    }

    fn size_hint(&self) -> usize {
        self.0.as_str().len()
    }
}

impl<'r> Decode<'r, Postgres> for Url {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <String as Decode<Postgres>>::decode(value)?;
        if let Ok(url) = url::Url::parse(&value) {
            Ok(Self(url))
        } else {
            Err(InvalidUrl.into())
        }
    }
}

use std::fmt;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

#[derive(Debug)]
struct InvalidUrl;

impl fmt::Display for InvalidUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid url")
    }
}

impl std::error::Error for InvalidUrl {}

impl From<String> for Url {
    fn from(value: String) -> Self {
        let url = url::Url::parse(&value).expect("url to be ok");
        Self(url)
    }
}
