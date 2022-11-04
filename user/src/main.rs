use std::{
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use clap::Parser;
use proto::{Message, Peer};

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
    let puncher_addr = (args.nameserver, args.port)
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
    let mut buf = Vec::from([Message::Request as u8]);
    buf.extend_from_slice(name.as_bytes());
    puncher.send_to(&buf, addr).expect("couldn't send message");

    // Receive server address or error from a puncher
    puncher.set_read_timeout(Some(Duration::new(5, 0)))?;
    let mut buf = [0u8; 1024];
    let len = puncher.recv(&mut buf).expect("receive from server");
    assert_eq!(Some(Message::ServerAddress), Message::from_repr(buf[0]));
    let server = rmp_serde::from_slice::<Peer>(&buf[1..len]).unwrap();

    println!("Connecting to server: {}", server.address);
    let mut buf = Vec::from([Message::Message as u8]);
    buf.extend_from_slice("This is a message from a client!".as_bytes());
    puncher
        .send_to(&buf, server.address)
        .expect("couldn't send data");
    let mut buf = [0u8; 1024];
    let len = puncher.recv(&mut buf)?;
    println!("Received from server: {:?}", &buf[..len]);

    let mut buffer = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        puncher
            .send_to(buffer.as_bytes(), server.address)
            .expect("couldn't send data");
    }

    Ok(())
}

fn start_server(puncher: UdpSocket, addr: SocketAddr, name: &str) -> std::io::Result<()> {
    let mut buf = Vec::from([Message::Announcement as u8]);
    buf.extend_from_slice(name.as_bytes());
    puncher.send_to(&buf, addr).expect("couldn't send message");

    loop {
        let mut buf = [0; 1024];
        let (len, src) = puncher.recv_from(&mut buf)?;

        if Some(Message::ClientAddress) == Message::from_repr(buf[0]) {
            // Client address received, punching hole to the client, since
            // he should already send us a welcome packet
            let client = rmp_serde::from_slice::<Peer>(&buf[1..len]).unwrap();

            // 0xAA doesn't mean anything we just need to send something in order to fully traverse NAT
            puncher
                .send_to(&[0xAA], client.address)
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
