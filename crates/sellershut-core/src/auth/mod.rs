use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[non_exhaustive]
pub enum OauthProvider {
    /// Discord
    Discord,
    /// Google
    Google,
}

impl OauthProvider {
    pub fn scopes(&self) -> Vec<String> {
        match self {
            OauthProvider::Discord => vec!["identify".into(), "email".into()],
            OauthProvider::Google => todo!(),
        }
    }
    pub fn cookie_name(&self) -> String {
        format!("auth_oauth_state_{self}")
    }
}

impl Display for OauthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OauthProvider::Discord => "discord",
                OauthProvider::Google => "google",
            }
        )
    }
}

impl FromStr for OauthProvider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "discord" => Ok(Self::Discord),
            "google" => Ok(Self::Google),
            _ => Err(()),
        }
    }
}
