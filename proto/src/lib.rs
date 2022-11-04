use std::net::SocketAddr;

use enum_repr::EnumRepr;
use serde::{Deserialize, Serialize};

#[EnumRepr(type = "u8", implicit = true)]
#[derive(PartialEq, Eq, Debug)]
pub enum Message {
    Error,
    Message,
    Announcement,
    Request,
    ClientAddress,
    ServerAddress,
}

/// IP and port of client
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    pub address: SocketAddr,
}

impl Peer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
}
