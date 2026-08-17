use std::collections::{HashMap, HashSet};

use sellershut_core::auth::OauthProvider;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Server {
    pub port: Port,
    pub request: Request,
    pub cors: Cors,
    pub oauth: OauthConfig,
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct Port(u16);

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct OauthConfig(pub HashMap<OauthProvider, sellershut_auth::Configuration>);

impl Default for OauthConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(OauthProvider::Discord, Default::default());
        Self(map)
    }
}

impl Default for Port {
    fn default() -> Self {
        Self(2210)
    }
}

impl From<Port> for u16 {
    fn from(value: Port) -> Self {
        value.0
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case", default)]
pub struct Request {
    pub timeout_duration: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case", default)]
pub struct Cors {
    pub allowed_origins: Vec<Url>,
    pub allowed_methods: AllowedMethods,
}

impl Default for Cors {
    fn default() -> Self {
        Self {
            allowed_origins: vec![Url::parse("http://localhost:5173").expect("valid url")],
            allowed_methods: Default::default(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AllowedMethods(HashSet<Method>);

impl From<&AllowedMethods> for Vec<axum::http::Method> {
    fn from(value: &AllowedMethods) -> Self {
        let mut methods = Vec::with_capacity(value.0.len());
        for method in &value.0 {
            methods.push(match method {
                Method::Get => axum::http::Method::GET,
                Method::Post => axum::http::Method::POST,
                Method::Put => axum::http::Method::PUT,
                Method::Delete => axum::http::Method::DELETE,
                Method::Patch => axum::http::Method::PATCH,
                Method::Head => axum::http::Method::HEAD,
            });
        }
        methods
    }
}

impl Default for AllowedMethods {
    fn default() -> Self {
        let set = HashSet::from_iter([
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Patch,
            Method::Head,
        ]);

        Self(set)
    }
}

#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            timeout_duration: 5,
        }
    }
}
