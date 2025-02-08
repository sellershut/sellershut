use std::net::{Ipv6Addr, SocketAddr};

use anyhow::Result;

#[derive(Clone)]
pub struct AppState {
    pub addr: SocketAddr,
}

impl AppState {
    pub async fn new(port: u16) -> Result<Self> {
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

        Ok(Self {
            addr: listen_address,
        })
    }
}
