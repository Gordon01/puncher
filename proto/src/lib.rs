use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[repr(u8)]
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
            0..=5 => unsafe { std::mem::transmute(t) },
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
