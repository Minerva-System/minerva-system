//! This module contains all implementations of fairings.
//!
//! Fairings are Rocket's approach to structured middlewares. These fairings
//! can hook into some parts of a request's lifecycle, and can be used for
//! operations such as user authentication, for example.

pub mod auth;
