//! This is a library for working with anything related to cache on the Minerva
//! System.

#![warn(clippy::all)]
#![warn(missing_docs)]

use redis::{Client, RedisResult};

pub mod auth;

pub fn build_client_string(server: &str) -> String {
    format!("redis://{}/", server)
}

pub fn build_client(server: &str) -> RedisResult<Client> {
    Client::open(build_client_string(server))
}
