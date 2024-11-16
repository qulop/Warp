use std::io::{Result, Error, ErrorKind};
use std::ops::RangeInclusive;
use std::net::{TcpListener, Ipv4Addr};


mod details {
    pub mod win32 {             
        use windows::Win32::Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS};
        use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_IN};
        use windows::Win32::NetworkManagement::IpHelper::{
            GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX, 
            GET_ADAPTERS_ADDRESSES_FLAGS, IP_ADAPTER_ADDRESSES_LH,
            IP_ADAPTER_UNICAST_ADDRESS_LH
        };

        use std::io::{ErrorKind, Error, Result};
        use std::net::Ipv4Addr;
        use log::debug;
        use std::ptr;



        unsafe fn get_adapters_addresses(adapters_list: &Option<*mut IP_ADAPTER_ADDRESSES_LH>, buf_sz: *mut u32) -> u32 {
            const FAMILY: u32 = AF_INET.0 as u32;
            const FLAGS: GET_ADAPTERS_ADDRESSES_FLAGS = GAA_FLAG_INCLUDE_PREFIX;

            return GetAdaptersAddresses(FAMILY, FLAGS, Some(ptr::null_mut()), 
                    *adapters_list, buf_sz);
        }

        unsafe fn reallocate_adapters_list(buffer: &mut Vec<u8>, adapters_list: &mut *mut IP_ADAPTER_ADDRESSES_LH, out_buf_size: &u32) {
            buffer.resize(*out_buf_size as usize, 0);

            *adapters_list = buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;
        }


        pub fn get_local_ipv4() -> Result<Ipv4Addr> {
            unsafe {
                let mut buf_sz: u32 = 1024;
                let mut buffer: Vec<u8> = vec![0; buf_sz as usize];
                let mut adapters_list: *mut IP_ADAPTER_ADDRESSES_LH = buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

                let mut result: u32 = get_adapters_addresses(&Some(adapters_list), &mut buf_sz);
                if result == ERROR_BUFFER_OVERFLOW.0 {
                    buf_sz *= 2;
                    reallocate_adapters_list(&mut buffer, &mut adapters_list, &buf_sz);
                    result = get_adapters_addresses(&Some(adapters_list), &mut buf_sz);
                }

                if result != ERROR_SUCCESS.0 || adapters_list.is_null() {
                    return Err(Error::new(ErrorKind::NotFound, "Failed to retrieve IPv4 address(Win32)"));
                }
                
                let mut adapter: *const IP_ADAPTER_ADDRESSES_LH = adapters_list;
                let mut ip_addr: u32 = 0;

                while !adapter.is_null() {
                    debug!("Retrieved adapter:\n\tAdapter Name: {}\n\tDescription: {}",
                        (*adapter).AdapterName.to_string().unwrap(),
                        (*adapter).Description.to_string().unwrap());


                    let unicast_addr: *mut IP_ADAPTER_UNICAST_ADDRESS_LH = (*adapter).FirstUnicastAddress;
                    if unicast_addr.is_null() {
                        adapter = (*adapter).Next;
                        continue;
                    }

                    let socket_addr: *const SOCKADDR_IN = (*unicast_addr).Address.lpSockaddr as *const SOCKADDR_IN;
                    ip_addr = (*socket_addr).sin_addr.S_un.S_addr;
                    break;
                }

                return Ok(Ipv4Addr::from_bits(ip_addr));
            }
        }
    }


    pub mod unix {
        use std::net::Ipv4Addr;
        use std::io::{Error, ErrorKind, Result};

        pub fn get_local_ipv4() -> Result<Ipv4Addr> { 
            return Err(Error::new(ErrorKind::NotFound, "Some"));
        }
    }
}




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
    
    panic!("Implementation for your operation system doesn't exist!");
}