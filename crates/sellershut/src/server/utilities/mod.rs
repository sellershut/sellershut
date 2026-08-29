pub mod create;

use sellershut_utilities::users::validate_username;
use url::Url;

use uuid::Uuid;

#[derive(Clone)]
pub struct ActivityPubIds<'a> {
    base_url: Url,
    username: &'a str,
}

impl<'a> ActivityPubIds<'a> {
    pub fn new(port: u16, domain: &str, user: &'a str) -> Result<Self, anyhow::Error> {
        if !validate_username(user) {
            return Err(anyhow::anyhow!("invalid username"));
        }
        Ok(Self {
            base_url: if cfg!(debug_assertions) {
                Url::parse(&format!("http://localhost:{port}/"))?
            } else {
                Url::parse(&format!("https://{domain}/"))?
            },
            username: user,
        })
    }

    pub fn inbox(&self) -> Result<Url, anyhow::Error> {
        self.build(&["users", self.username, "inbox"])
    }

    pub fn outbox(&self) -> Result<Url, anyhow::Error> {
        self.build(&["users", self.username, "outbox"])
    }

    pub fn activity(&self) -> Result<Url, anyhow::Error> {
        self.build(&[
            "users",
            self.username,
            "activities",
            &Uuid::now_v7().to_string(),
        ])
    }

    pub fn object(&self) -> Result<Url, anyhow::Error> {
        self.build(&[
            "users",
            self.username,
            "objects",
            &Uuid::now_v7().to_string(),
        ])
    }

    pub fn users(&self) -> Result<Url, anyhow::Error> {
        self.build(&["users", self.username])
    }

    pub fn followers(&self) -> Result<Url, anyhow::Error> {
        self.build(&["users", self.username, "followers"])
    }

    pub fn following(&self) -> Result<Url, anyhow::Error> {
        self.build(&["users", self.username, "following"])
    }

    pub fn likes(&self) -> Result<Url, anyhow::Error> {
        self.build(&["users", self.username, "likes"])
    }

    fn build(&self, segments: &[&str]) -> Result<Url, anyhow::Error> {
        let mut url = self.base_url.clone();

        {
            let mut path = url.path_segments_mut().map_err(|_| {
                anyhow::anyhow!("Base URL is not hierarchical and cannot accept path segments")
            })?;

            path.clear();

            for segment in segments {
                path.push(segment);
            }
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_base_url() {
        let base = ActivityPubIds::new(8080, "example.com", "").unwrap();
        assert_eq!(base.base_url.as_str(), "http://localhost:8080/");
    }

    #[test]
    fn check_outbox_url() {
        let base = ActivityPubIds::new(8080, "example.com", "alice").unwrap();
        assert_eq!(
            base.outbox().unwrap().as_str(),
            "http://localhost:8080/users/alice/outbox"
        );
    }

    #[test]
    fn check_inbox_url() {
        let base = ActivityPubIds::new(8080, "example.com", "alice").unwrap();
        assert_eq!(
            base.inbox().unwrap().as_str(),
            "http://localhost:8080/users/alice/inbox"
        );
    }

    #[test]
    fn check_users_url() {
        let base = ActivityPubIds::new(8080, "example.com", "alice").unwrap();
        assert_eq!(
            base.users().unwrap().as_str(),
            "http://localhost:8080/users/alice"
        );
    }

    #[test]
    fn invalid_username() {
        let base = ActivityPubIds::new(8080, "example.com", "alice     ");
        assert!(base.is_err());
    }
}
