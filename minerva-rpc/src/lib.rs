//! This is a library for working with anything related to gRPC communication on
//! the Minerva System. This includes the implementation of client and server
//! codes for each service, plus the DTOs representing messages sent to and from
//! remote procedure calls.

#![warn(clippy::all)]
#![warn(missing_docs)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::Request;

/// Returns the address of any request's sender. If non-existing, returns
/// and IPv4 address of `0.0.0.0`.
pub fn get_address<T>(req: &Request<T>) -> SocketAddr {
    req.remote_addr()
        .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
}

pub mod metadata;
pub mod products;
pub mod session;
pub mod users;

#[allow(missing_docs)]
pub mod messages {
    tonic::include_proto!("messages");
}
