use secrecy::{ExposeSecret, SecretString};
use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RedactedSecret(SecretString);

impl RedactedSecret {
    pub fn expose(&self) -> String {
        self.0.expose_secret().to_owned()
    }
}

impl From<String> for RedactedSecret {
    fn from(value: String) -> Self {
        Self(SecretString::from(value))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RedactedSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("")
    }
}

impl Type<Postgres> for RedactedSecret {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <String as Type<Postgres>>::compatible(ty)
    }
}

impl<'q> Encode<'q, Postgres> for RedactedSecret {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        <String as Encode<Postgres>>::encode_by_ref(&self.0.expose_secret().to_owned(), buf)
    }

    fn size_hint(&self) -> usize {
        self.0.expose_secret().len()
    }
}

impl<'r> Decode<'r, Postgres> for RedactedSecret {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <String as Decode<Postgres>>::decode(value)?;

        Ok(value.into())
    }
}

#[cfg(test)]
mod tests;
