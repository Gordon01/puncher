use std::{net::{UdpSocket, SocketAddr, Ipv4Addr}, time::Duration};

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

    let puncher = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    puncher
        .connect("178.62.1.149:4200")
        .expect("connect function failed");

    match args.mode {
        Mode::Server => start_server(puncher, &args.name).expect("start server"),
        Mode::Client => client_request(puncher, &args.name).expect("perform client request"),
    }

    Ok(())
}

fn client_request(puncher: UdpSocket, name: &str) -> std::io::Result<()> {
    // 0x01 - Client announcement
    let mut buf = Vec::from([0x01]);
    buf.extend_from_slice(name.as_bytes());
    puncher.send(&buf).expect("couldn't send message");

    // Receive server address or error from a puncher
    puncher.set_read_timeout(Some(Duration::new(5, 0)))?;
    let len = puncher.recv(&mut buf).expect("receive from server");
    let port = u16::from_be_bytes([buf[len-2], buf[len-1]]);
    let ip = Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]);
    let server_address = SocketAddr::from((ip, port));
    
    // Create a new socket to server
    println!("Connecting to server: {server_address}");
    let server = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");
    let string = String::from("This is a message from a client!");
    server.connect(server_address)?;
    server.send(string.as_bytes()).expect("couldn't send data");
    let mut buf = [0u8; 1024];
    server.recv(&mut buf)?;
    println!("Server said: {}", std::str::from_utf8(&buf).unwrap());

    Ok(())
}

fn start_server(puncher: UdpSocket, name: &str) -> std::io::Result<()> {
    // 0x00 - Server announcement
    let mut buf = Vec::from([0x00]);
    buf.extend_from_slice(name.as_bytes());
    puncher.send(&buf).expect("couldn't send message");

    loop {
        let mut buf = [0; 1024];
        let (len, src) = puncher.recv_from(&mut buf)?;
        let message = std::str::from_utf8(&buf[..len]).unwrap();
        println!("Peer: {src}, length: {len} bytes", );
        println!("{message}");

        let string = String::from("Server says hi!");
        puncher.send_to(string.as_bytes(), src).expect("couldn't send data");
    }
}
