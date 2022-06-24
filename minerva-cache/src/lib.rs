//! This is a library for working with anything related to cache on the Minerva
//! System.

#![warn(clippy::all)]
#![warn(missing_docs)]

pub mod auth;

pub fn build_client_string(server: &str) -> String {
    format!("redis://{}/", server)
}
