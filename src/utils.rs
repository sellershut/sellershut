use anyhow::Result;
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
    let id = generate_id(length);
    Url::parse(&format!("{}/objects/{}", domain, id))
}

use nanoid::nanoid;
/// Alphabet of characters making up an ID
const ID_ALPHABET: [char; 36] = [
    '2', '3', '4', '5', '6', '7', '8', '9', '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '-',
];

/// Length of characters in ID
pub const ID_LENGTH: usize = 21;

/// Generates a nanoid (21 characters)
pub fn generate_id(length: usize) -> String {
    nanoid!(length, &ID_ALPHABET)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_in_id(character: char, expected_result: bool) -> String {
        let id = generate_id(ID_LENGTH);
        let actual_result = id.contains(character);
        assert_eq!(expected_result, actual_result);
        id
    }

    #[test]
    fn check_valid() {
        check_in_id('1', false);
        let id = check_in_id('0', false);

        assert_eq!(ID_LENGTH, id.len());
    }
}
