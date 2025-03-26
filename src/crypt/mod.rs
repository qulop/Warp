use std::net::Ipv4Addr;



#[allow(non_camel_case_types)]
type byte = u8;

pub struct CryptProps {
    key: &'static [u8],
    iv: &'static [u8]
}

impl CryptProps {
    pub fn new() -> Self {
        return Self {
            key: b"some key",
            iv: b"some iv"
        }
    }
}

pub fn encrypt_host_data(ip: Ipv4Addr, port: u16) -> Vec<byte> {
    todo!();
}


pub fn decrypt_host_data(ecrypted_data: &str) -> Option<(Ipv4Addr, u16)> {
    let props: CryptProps = CryptProps::new();

    return None;
}

// IP:PORT/PASSCODE