use std::ops::Deref;

use sellershut_core::{RedactedSecret, auth::s::serialize_redacted_secret};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case", default)]
pub struct Vault {
    pub url: VaultUrl,
    #[serde(serialize_with = "serialize_redacted_secret")]
    pub token: RedactedSecret,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            url: Default::default(),
            token: String::from("devtoken").into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VaultUrl(Url);

impl Deref for VaultUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for VaultUrl {
    fn default() -> Self {
        Self(Url::parse("http://localhost:8200").expect("valid url"))
    }
}
