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

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn check_parse(input: &str, expected_result: Result<OauthProvider, ()>) {
        let actual_result = input.parse::<OauthProvider>();
        assert_eq!(expected_result, actual_result);
    }

    #[track_caller]
    fn check_display(input: OauthProvider, expected_result: &str) {
        let actual_result = input.to_string();
        assert_eq!(expected_result, actual_result);
    }

    #[track_caller]
    fn check_scopes(input: OauthProvider, expected_result: &[&str]) {
        let actual_result = input.scopes();
        let expected_result = expected_result
            .iter()
            .map(|scope| (*scope).to_string())
            .collect::<Vec<_>>();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn parse_empty() {
        check_parse("", Err(()));
    }

    #[test]
    fn parse_unknown() {
        check_parse("github", Err(()));
        check_parse("Discord", Err(()));
        check_parse("GOOGLE", Err(()));
    }

    #[test]
    fn parse_known() {
        check_parse("discord", Ok(OauthProvider::Discord));
        check_parse("google", Ok(OauthProvider::Google));
    }

    #[test]
    fn display() {
        check_display(OauthProvider::Discord, "discord");
        check_display(OauthProvider::Google, "google");
    }

    #[test]
    fn display_parse_round_trip() {
        check_parse(
            &OauthProvider::Discord.to_string(),
            Ok(OauthProvider::Discord),
        );

        check_parse(
            &OauthProvider::Google.to_string(),
            Ok(OauthProvider::Google),
        );
    }

    #[test]
    fn discord_scopes() {
        check_scopes(OauthProvider::Discord, &["identify", "email"]);
    }
}
