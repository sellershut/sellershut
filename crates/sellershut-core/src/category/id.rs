use std::{fmt, str::FromStr};

use ulid::Ulid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CategoryId(String);

impl CategoryId {
    pub fn new() -> Self {
        Self(Ulid::generate().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_ulid(&self) -> Ulid {
        Ulid::from_string(&self.0).expect("CategoryId should always contain a valid ULID")
    }
}

impl Default for CategoryId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CategoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<Ulid> for CategoryId {
    fn from(value: Ulid) -> Self {
        Self(value.to_string())
    }
}

impl FromStr for CategoryId {
    type Err = ulid::DecodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let ulid = Ulid::from_string(value)?;
        Ok(Self(ulid.to_string()))
    }
}
