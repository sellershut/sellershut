#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Url(url::Url);

impl From<url::Url> for Url {
    fn from(value: url::Url) -> Self {
        Self(value)
    }
}

impl From<&url::Url> for Url {
    fn from(value: &url::Url) -> Self {
        Self(value.clone())
    }
}

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

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidUrl {}

impl From<String> for Url {
    fn from(value: String) -> Self {
        let url = url::Url::parse(&value).expect("url to be ok");
        Self(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check_from(input: &str, expected_result: &str) {
        let actual_result = Url::from(input.to_owned());

        assert_eq!(expected_result, actual_result.inner().as_str());
    }

    #[track_caller]
    fn check_size_hint(input: &str, expected_result: usize) {
        let url = Url::from(input.to_owned());

        assert_eq!(expected_result, url.size_hint());
    }

    #[test]
    fn from_string() {
        check_from("https://example.com", "https://example.com/");

        check_from("https://example.com/foo", "https://example.com/foo");

        check_from(
            "https://example.com/foo?bar=baz",
            "https://example.com/foo?bar=baz",
        );
    }

    #[test]
    fn size_hint() {
        check_size_hint("https://example.com", "https://example.com/".len());

        check_size_hint("https://example.com/foo", "https://example.com/foo".len());
    }

    #[test]
    fn is_str() {
        assert!(<Url as Type<Postgres>>::compatible(&<String as Type<
            Postgres,
        >>::type_info(
        ),));
    }

    #[test]
    fn can_enconde() {
        let url = Url(url::Url::parse("http://example.com").expect("url"));
        let mut buf = PgArgumentBuffer::default();
        let result = url.encode_by_ref(&mut buf);

        assert!(result.is_ok());
        assert!(!buf.is_empty());
    }
}
