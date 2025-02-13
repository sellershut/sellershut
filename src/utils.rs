use anyhow::Result;
use url::Url;

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
