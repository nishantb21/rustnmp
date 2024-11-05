use std::net::{UdpSocket, SocketAddr};
use std::io;

struct DataGram {
    
}

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

        for message_component in buf.iter() {
            print!("{:x} ", message_component);
        }
        println!();
    }
}
