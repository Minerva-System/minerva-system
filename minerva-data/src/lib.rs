//! This is a library for working with anything related to actual data on
//! the project. This includes:
//!
//! - Data Transfer Objects (DTOs) for any defined entities;
//! - Conversions from gRPC messages to actual entity DTOs;
//! - Loading and reading project configuration such as multi-tenant config;
//! - Logging to screen (non-audit logging).

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
extern crate num_derive;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod encryption;
pub mod file;
pub mod log;
#[allow(unused_imports)]
pub mod schema;
pub mod syslog;
pub mod tenancy;
pub mod user;
