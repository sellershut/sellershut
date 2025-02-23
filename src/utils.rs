use anyhow::Result;
use sellershut_services::utils;
use url::{ParseError, Url};

pub fn get_domain_with_port(url_str: &str) -> Result<String> {
    let url = Url::parse(url_str)?;
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("host str unavailable"))?;

    let port = url.port_or_known_default();

    if let Some(port) = port {
        Ok(format!("{}:{}", host, port))
    } else {
        Ok(host.to_string())
    }
}

pub fn generate_object_id(domain: &str, length: usize) -> Result<Url, ParseError> {
    let id = utils::generate_id(length);
    Url::parse(&format!("{}/objects/{}", domain, id))
}
