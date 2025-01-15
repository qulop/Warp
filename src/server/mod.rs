use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use log::debug;
use scrap::{Capturer, Display};

use crate::networking::{find_available_port, is_port_available, get_local_ipv4};
use crate::utils::result_single::ResultSingle;


struct Server {
    socket: UdpSocket,
    buffer: Vec<u8>,
    capturer: Capturer
}

impl Server {
    fn new(ip: &Ipv4Addr, port: u16) -> Option<Self> {
        let sock_addr = SocketAddrV4::new(*ip, port);

        let socket: UdpSocket = UdpSocket::bind(sock_addr)
            .expect(format!("Failed to bind server to UdpSocket. IP: {ip}, port: {port}")
                .as_str());


        let display: Display = Display::primary().ok()?;
        let capturer: Capturer = Capturer::new(display).ok()?;

        return Some(Self {
            socket,
            buffer: vec![0 as u8; 1024],
            capturer,
        })
    }


    pub async fn start_listening(&mut self) -> ResultSingle<()> {

        loop {
            match self.capturer.frame() {
                Ok(o) => {

                }

                _ => {

                }
            } 
        }


        return Ok(());   
    }

    async fn send_frame(&mut self) -> ResultSingle<()> {
        todo!();
    }
}


pub async fn server_main() -> ResultSingle<()> {
    let local_ip: Ipv4Addr = get_local_ipv4()?;
    let server_port: u16 = is_port_available(9516)
        .or(find_available_port(7700..=15000))?;
    
    debug!("Retrieved local IP address: {local_ip}. Selected server port: {server_port}");
    
    let mut server: Server = Server::new(&local_ip, server_port).unwrap();
    debug!("Server instance was created...");

    return server.start_listening().await;
}