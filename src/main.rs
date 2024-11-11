use std::io::Error;
use std::net::{SocketAddr, UdpSocket};
use std::env::args;



struct Server {
    socket: UdpSocket,
    buffer: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>
}

impl Server {
    fn new(ip: &String) -> Self {
        let socket = UdpSocket::bind(ip)
            .expect("Failed to bind specified ip address");

        return Self {
            socket, 
            buffer: vec![0; 1024], 
            to_send: None
        }
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            if let Some((size, peer)) = self.to_send {
                let amount = self.socket.send_to(&self.buffer[..size], &peer).unwrap();

                println!("Echoed {amount}/{size} bytes to {peer}");
            }
        }
    }
}


fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if let None = args.get(1) {
        return Ok(());
    }

    let ipaddr: &String = args.get(1).unwrap();
    let mut server = Server::new(&ipaddr);
    server.run()?;

    return Ok(());
}