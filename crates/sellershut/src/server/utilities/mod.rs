use sellershut_utilities::users::validate_username;
use url::Url;

pub fn base_url(port: u16, domain: &str) -> Result<Url, url::ParseError> {
    if cfg!(debug_assertions) {
        Url::parse(&format!("http://localhost:{port}/"))
    } else {
        Url::parse(&format!("https://{domain}/"))
    }
}

pub fn inbox_url(port: u16, domain: &str, username: &str) -> Result<Url, url::ParseError> {
    base_url(port, domain)?.join(&format!("users/{username}/inbox"))
}

pub fn users_url(port: u16, domain: &str, username: &str) -> anyhow::Result<Url> {
    if validate_username(username) {
        Ok(base_url(port, domain)?.join(&format!("users/{username}"))?)
    } else {
        Err(anyhow::anyhow!("invalid username"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_base_url() {
        assert_eq!(
            base_url(8080, "example.com").unwrap().as_str(),
            "http://localhost:8080/"
        );
    }

    #[test]
    fn check_inbox_url() {
        assert_eq!(
            inbox_url(8080, "example.com", "alice").unwrap().as_str(),
            "http://localhost:8080/users/alice/inbox"
        );
    }

    #[test]
    fn check_users_url() {
        assert_eq!(
            users_url(8080, "example.com", "alice").unwrap().as_str(),
            "http://localhost:8080/users/alice"
        );
    }

    #[test]
    fn invalid_username() {
        assert_eq!(
            users_url(8080, "example.com", "alice   ").unwrap().as_str(),
            "http://localhost:8080/users/alice"
        );
    }
}
