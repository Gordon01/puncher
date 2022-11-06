use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use enum_repr::EnumRepr;
use serde::{Deserialize, Serialize};

#[EnumRepr(type = "u8", implicit = true)]
#[derive(PartialEq, Eq, Debug)]
pub enum Message {
    /// Error description is send as UTF8 string
    Error,
    /// Informational UTF8 string
    Message,
    /// Server announces itself
    Announcement,
    /// Peer requests server address if a server
    Request,
    ClientAddress,
    ServerAddress,
}

/// IP and port of client
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Peer {
    pub address: SocketAddr,
}

impl Peer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
}

pub struct Packet {
    data: Vec<u8>,
}

impl Packet {
    pub fn new(message: Message) -> Self {
        Self {
            data: Vec::from([message.repr()]),
        }
    }

    pub fn add_raw_data(mut self, data: &[u8]) -> Self {
        self.data.extend(data);
        self
    }

    pub fn add_addr(self, addr: &SocketAddr) -> Self {
        let serialized = rmp_serde::to_vec(&Peer::new(addr.to_owned())).unwrap();
        self.add_raw_data(&serialized)
    }

    pub fn send(self, socket: &UdpSocket, dst: impl ToSocketAddrs) -> std::io::Result<usize> {
        socket.send_to(&self.data, dst)
    }
}
