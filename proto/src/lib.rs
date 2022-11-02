use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub enum Message {
    Error,
    Announcement,
    Request,
    ClientAddress,
    ServerAddress,
    UnsupportedType,
}

impl From<u8> for Message {
    fn from(t: u8) -> Self {
        match t {
            0 => Message::Error,
            1 => Message::Announcement,
            2 => Message::Request,
            3 => Message::ClientAddress,
            4 => Message::ServerAddress,
            _ => Message::UnsupportedType,
        }
    }
}

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
