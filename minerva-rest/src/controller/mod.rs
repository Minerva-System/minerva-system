//! This module is responsible for encapsulating the controllers for the
//! entities. These controllers are mostly responsible for defining the REST
//! endpoints for them.
//!
//! This module also encapsulates common HTTP handlers for several HTTP status
//! codes.

pub mod auth;
pub mod handlers;
pub mod response;
pub mod user;
