use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::time::{timeout, Duration};
use tokio::net::UdpSocket as TokioUdpSocket;


#[derive(Debug)]
pub struct ClientConnection {
    socket: TokioUdpSocket,
    host_address: SocketAddr
}

impl ClientConnection {
    pub async fn new(host_data: &String) -> Option<Self> {
        let parts: Vec<&str> = host_data.split(':').collect();
        if parts.len() > 2 {
            return None;
        }

        let host_ip = parts[0].parse::<Ipv4Addr>().ok()?;
        let host_port = parts[1].parse::<u16>().ok()?;

        let remote_addr = SocketAddr::new(IpAddr::V4(host_ip), host_port);

        let sock = TokioUdpSocket::bind("0.0.0.0:0").await.ok()?;
        sock.connect(remote_addr).await.ok()?;
        
        return Some(Self {
            socket: sock,
            host_address: remote_addr
        });
    }

    pub async fn try_connect(&self, passcode: &str) -> Option<()> {
        let ask_packet: String = format!("CON\0{passcode}");
        self.socket.send_to(ask_packet.as_bytes(), &self.host_address).await;

        let mut buf =  [0 as u8; 1024];
        match timeout(Duration::from_secs(5), self.socket.recv_from(&mut buf)).await {
            Ok(Ok((len, _))) => {
                let response = String::from_utf8_lossy(&buf[..len]);
                if response == "OK" {
                    return Some(());
                }
                return None;
            }
            _ => return None
        }
    }
}
