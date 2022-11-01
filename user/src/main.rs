use std::{
    net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Server,
    Client,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host name of the Nameserver
    #[arg(long, default_value = "allnotify.ru")]
    nameserver: String,

    /// Port to listen to
    #[arg(short, long, default_value_t = 4200)]
    port: u16,

    /// Name
    #[arg(short, long)]
    name: String,

    /// Mode
    #[arg(short, long)]
    mode: Mode,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Connecting to {}:{}", args.nameserver, args.port);

    let puncher = UdpSocket::bind("0.0.0.0:2288").expect("couldn't bind to address");
    let puncher_addr = "178.62.1.149:4200"
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    match args.mode {
        Mode::Server => start_server(puncher, puncher_addr, &args.name).expect("start server"),
        Mode::Client => {
            client_request(puncher, puncher_addr, &args.name).expect("perform client request")
        }
    }

    Ok(())
}

fn client_request(puncher: UdpSocket, addr: SocketAddr, name: &str) -> std::io::Result<()> {
    // 0x01 - Client announcement
    let mut buf = Vec::from([0x01]);
    buf.extend_from_slice(name.as_bytes());
    puncher.send_to(&buf, addr).expect("couldn't send message");

    // Receive server address or error from a puncher
    puncher.set_read_timeout(Some(Duration::new(5, 0)))?;
    puncher.recv(&mut buf).expect("receive from server");
    let server_address = address_from_bytes(&buf);

    // Create a new socket to server
    println!("Connecting to server: {server_address}");
    let string = String::from("This is a message from a client!");
    puncher
        .send_to(string.as_bytes(), server_address)
        .expect("couldn't send data");
    let mut buf = [0u8; 1024];
    let len = puncher.recv(&mut buf)?;
    println!("Received from server: {:?}", &buf[..len]);

    let mut buffer = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        puncher
            .send_to(buffer.as_bytes(), server_address)
            .expect("couldn't send data");
    }

    Ok(())
}

fn start_server(puncher: UdpSocket, addr: SocketAddr, name: &str) -> std::io::Result<()> {
    // 0x00 - Server announcement
    let mut buf = Vec::from([0x00]);
    buf.extend_from_slice(name.as_bytes());
    puncher.send_to(&buf, addr).expect("couldn't send message");

    loop {
        let mut buf = [0; 1024];
        let (len, src) = puncher.recv_from(&mut buf)?;

        if buf[0] == 0 {
            // Client address received, punching hole to the client, since
            // he should already send us a welcome packet
            let client_address = address_from_bytes(&buf[1..]);

            // 0xAA doesn't mean anything we just need to send something in order to fully traverse NAT
            puncher
                .send_to(&[0xAA], client_address)
                .expect("punching client");
            continue;
        }

        let message = std::str::from_utf8(&buf[..len]).unwrap();
        println!("Peer: {src}, length: {len} bytes",);
        println!("{message}");

        let string = String::from("Server says hi!");
        puncher
            .send_to(string.as_bytes(), src)
            .expect("couldn't send data");
    }
}

fn address_from_bytes(buffer: &[u8]) -> SocketAddr {
    let port = u16::from_be_bytes([buffer[4], buffer[5]]);
    let ip = Ipv4Addr::new(buffer[0], buffer[1], buffer[2], buffer[3]);

    SocketAddr::from((ip, port))
}
