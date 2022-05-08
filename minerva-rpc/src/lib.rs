use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::Request;

pub fn get_address<T>(req: &Request<T>) -> SocketAddr {
    req.remote_addr()
        .unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0))
}

pub mod metadata;
pub mod products;
pub mod users;

pub mod messages {
    tonic::include_proto!("messages");
}
