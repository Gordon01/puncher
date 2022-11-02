use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug)]
pub enum Message {
    Error,
    Message,
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
            1 => Message::Message,
            2 => Message::Announcement,
            3 => Message::Request,
            4 => Message::ClientAddress,
            5 => Message::ServerAddress,
            _ => Message::UnsupportedType,
        }
    }
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
