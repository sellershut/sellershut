use auth::OauthConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct AuthConfig {
    pub discord: OauthConfig,
}
