use std::net::{SocketAddr, SocketAddrV4};
use log::debug;
use scrap::{Capturer, Display};

use tokio::net::{UdpSocket as TokioUdpSocket};

use crate::networking::{find_available_port, is_port_available, get_local_ipv4};
use crate::utils::result_single::ResultSingle;


#[derive(Debug)]
pub struct Server {
    socket: TokioUdpSocket,
    buffer: Vec<u8>,
    passcode: String,   // TODO
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    pub async fn new() -> Option<Self> {
        let ip = get_local_ipv4().ok()?;
        let port = is_port_available(9516)
            .or_else(|_| find_available_port(7700..=15000))
            .ok()?;

        let sock_addr = SocketAddrV4::new(ip, port);
        let socket = TokioUdpSocket::bind(sock_addr).await;
        if let Err(error) = socket {
            debug!("Failed to bind server to IP {ip} and port {port}: {error}");
            return None;
        }

        Some(Self {
            socket: socket.unwrap(),
            buffer: vec![0u8; 1024],
            passcode: String::new(),  // Инициализация passcode, TODO
            to_send: None,
        })
    }

    pub fn start_listening(&self) {

    } 

    fn check_recieved_passcode(&self, recieved_data: &[u8]) -> Option<()> {
        let recieved_data = String::from_utf8_lossy(recieved_data);

        return (recieved_data.starts_with("CON\0") && &recieved_data[4..recieved_data.len()] == self.passcode)
            .then_some(());
    }


    pub async fn star_listening_for_connection(&mut self) -> Option<()> {
        let mut buf = [0 as u8; 1024];

        match self.socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                self.check_recieved_passcode(&buf[..len])
                    .and_then(|_| async { 
                            self.socket.send_to(b"OK", addr).await.ok()?; 
                })
            }

            _ => {

            }
        }

    }

    async fn send_frame(&mut self) -> ResultSingle<()> {
        todo!();
    }
}
