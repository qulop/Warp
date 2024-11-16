mod details;

use std::io::{Result, Error, ErrorKind};
use std::ops::RangeInclusive;
use std::net::{TcpListener, Ipv4Addr};



pub fn is_port_available(port: u16) -> Result<u16> {
    return TcpListener::bind(("0.0.0.0", port)).map(|_| port);
}


pub fn find_available_port(mut rng: RangeInclusive<u16>) -> Result<u16> {
    match (rng).find(|port: &u16| is_port_available(*port).is_ok()) {
        Some(val) => return Ok(val),
        None => Err(Error::new(ErrorKind::NotFound, "No available port was found"))
    }
}


pub fn get_local_ipv4() -> Result<Ipv4Addr> {
    if cfg!(windows) || cfg!(unix) {
        if cfg!(windows) {
            return details::win32::get_local_ipv4();
        }
        return details::unix::get_local_ipv4();
    }
    
    panic!("Implementation of function `get_local_ipv4()` for your operation system doesn't exist!");
}