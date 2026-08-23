use std::ops::Deref;

use sellershut_core::RedactedSecret;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Vault {
    pub url: VaultUrl,
    pub token: RedactedSecret,
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
