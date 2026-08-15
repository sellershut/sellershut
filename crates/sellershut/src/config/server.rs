use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "kebab-case", default)]
pub struct Server {
    pub port: Port,
    pub request: Request,
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct Port(u16);

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

impl Default for Request {
    fn default() -> Self {
        Self {
            timeout_duration: 5,
        }
    }
}
