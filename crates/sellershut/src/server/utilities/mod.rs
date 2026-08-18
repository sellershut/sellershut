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
