use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr, UdpSocket},
};

use clap::Parser;
use proto::Peer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to listen to
    #[arg(short, long, default_value = "0.0.0.0")]
    ip: String,

    /// Port to listen to
    #[arg(short, long, default_value_t = 4200)]
    port: u16,
}

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let args = Args::parse();
    log::info!("Puncher is listening on {}:{}", args.ip, args.port);
    let socket = UdpSocket::bind((args.ip, args.port))?;
    let mut servers = HashMap::<String, SocketAddr>::new();

    loop {
        let mut buf = [0; 1024];
        let (len, src) = socket.recv_from(&mut buf)?;

        let message = buf[0];
        match message {
            0 => announcement(&buf[1..len], &mut servers, src),
            1 => request(&buf[1..len], &mut servers, src, &socket),
            _ => {
                log::error!("Message unsupported, type = {message}");
            }
        }
    }
}

fn announcement(data: &[u8], servers: &mut HashMap<String, SocketAddr>, source: SocketAddr) {
    let name = String::from_utf8(data.to_vec()).unwrap();
    log::info!("Adding {name} as {source}");
    servers.insert(name, source);
}

fn request(
    data: &[u8],
    servers: &mut HashMap<String, SocketAddr>,
    source: SocketAddr,
    socket: &UdpSocket,
) {
    let name = String::from_utf8(data.to_vec()).unwrap();
    let mut message = format!("Requested address of {name} by {source}, ");

    if let Some(addr) = servers.get(&name) {
        message.push_str(&format!("found: {addr}"));
        let mut ip = match addr.ip() {
            IpAddr::V4(ip) => ip.octets().to_vec(),
            IpAddr::V6(ip) => ip.octets().to_vec(),
        };
        ip.extend_from_slice(&addr.port().to_be_bytes());

        socket.send_to(&ip, source).expect("send server addr");

        // Also send to server client's address
        let mut ip = Vec::from([0]);
        ip.extend(match source.ip() {
            IpAddr::V4(ip) => ip.octets().to_vec(),
            IpAddr::V6(ip) => ip.octets().to_vec(),
        });
        ip.extend_from_slice(&source.port().to_be_bytes());
        socket.send_to(&ip, addr).expect("send client addr");
    } else {
        message.push_str("but not found");
    }

    log::info!("{message}");
}
