mod details;

use std::io::{Result, Error, ErrorKind};
use std::ops::RangeInclusive;
use std::net::{TcpListener, Ipv4Addr};


#[inline(always)]
pub fn is_port_available(port: u16) -> Result<u16> {
    return TcpListener::bind(("0.0.0.0", port)).map(|_| port);
}

#[inline]
pub fn find_available_port(mut rng: RangeInclusive<u16>) -> Result<u16> {
    match (rng).find(|port: &u16| is_port_available(*port).is_ok()) {
        Some(val) => return Ok(val),
        None => Err(Error::new(ErrorKind::NotFound, "No available port was found"))
    }
}

#[inline(always)]
pub fn get_local_ipv4() -> Result<Ipv4Addr> {
    #[cfg(target_os = "windows")] {
        return details::win32::get_local_ipv4();
    }
    
    #[cfg(target_os = "linux")] {
        return details::unix::get_local_ipv4();
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))] {
        unknown_platform!();
    }
}