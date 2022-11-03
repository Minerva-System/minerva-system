//! This module contains utilitary functions and structures that can be used by
//! other parts of this module that are somewhat global and can be reused
//! whenever needed.

use std::net::SocketAddr;

/// Returns the IP address and port of this application, which is always
/// `127.0.0.1:9000`.
pub fn get_ip() -> SocketAddr {
    use std::net::{IpAddr, Ipv4Addr};
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000)
}
