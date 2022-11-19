//! This module contains Data Transfer Objects for generic purposes.
#![allow(clippy::extra_unused_lifetimes)]

use rocket::response::Responder;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

/// DTO representing a generic message to represent anything.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Responder)]
pub struct Message {
    /// The message content.
    pub message: String,
}

impl From<&str> for Message {
    fn from(msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
        }
    }
}

impl From<String> for Message {
    fn from(msg: String) -> Self {
        Self {
            message: msg.clone(),
        }
    }
}
