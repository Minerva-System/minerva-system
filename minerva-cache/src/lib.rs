//! This is a library for working with anything related to cache on the Minerva
//! System.

#![warn(clippy::all)]
#![warn(missing_docs)]

use redis::{Client, RedisResult};

pub mod auth;

/// Generates a client connection string, given the server path
fn build_client_string(server: &str) -> String {
    format!("redis://{}/", server)
}

/// Generates a Redis client, given a server path.
pub fn build_client(server: &str) -> RedisResult<Client> {
    Client::open(build_client_string(server))
}
