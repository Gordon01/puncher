use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

/// IP and port of client
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    address: SocketAddr,
}

impl Peer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
}
