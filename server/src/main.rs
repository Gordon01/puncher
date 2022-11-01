use std::{net::{UdpSocket, SocketAddr, IpAddr}, collections::HashMap};

use clap::Parser;

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
    let args = Args::parse();
    println!("Listening on {}:{}", args.ip, args.port);
    let socket = UdpSocket::bind((args.ip, args.port))?;

    let mut servers = HashMap::<String, SocketAddr>::new();

    loop {
        let mut buf = [0; 1024];
        let (len, src) = socket.recv_from(&mut buf)?;
        let message = buf[0];

        println!("Peer: {:?}", src);
        println!("Message\tlength: {len} bytes");
        println!("\ttype: {}", message);

        if message == 0 {
            // Announcement
            let name = String::from_utf8(buf[1..len].to_vec()).unwrap();
            println!("Server name: {name}");

            servers.insert(name, src);
        } else if message == 1 {
            // Request
            let name = String::from_utf8(buf[1..len].to_vec()).unwrap();
            println!("Requested server name: {name}");

            if let Some(addr) = servers.get(&name) {
                println!("Found: {addr}");
                let mut ip = match addr.ip() {
                    IpAddr::V4(ip) => ip.octets().to_vec(),
                    IpAddr::V6(ip) => ip.octets().to_vec(),
                };
                ip.extend_from_slice(&addr.port().to_be_bytes());

                socket.send_to(&ip, src).expect("send server addr");

                // Also send to server client's address
                let mut ip = Vec::from([0]);
                ip.extend(match src.ip() {
                    IpAddr::V4(ip) => ip.octets().to_vec(),
                    IpAddr::V6(ip) => ip.octets().to_vec(),
                });
                ip.extend_from_slice(&src.port().to_be_bytes());
                socket.send_to(&ip, addr).expect("send client addr");
            }
        } else {
            eprintln!("Message unsupported, type = {message}");
        }
    }
}
