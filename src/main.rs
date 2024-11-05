use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::io;

fn main() -> io::Result<()> {
    // Define the address and port to listen on.
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("Invalid address");

    // Bind the socket to the address.
    let socket = UdpSocket::bind(addr)?;
    println!("Listening for UDP packets on {}", addr);

    let mut buf = [0u8; 1024];  // Buffer to store incoming data

    loop {
        // Receive data from the socket.
        let (bytes_received, src_addr) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", bytes_received, src_addr);

        // Convert the received bytes to a string (assuming it's UTF-8).
        if let Ok(msg) = str::from_utf8(&buf[..bytes_received]) {
            println!("Message: {}", msg);
        } else {
            println!("Received non-UTF8 data");
        }
    }
}
