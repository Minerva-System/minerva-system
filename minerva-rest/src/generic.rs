//! This module contains Data Transfer Objects for generic purposes.
#![allow(clippy::extra_unused_lifetimes)]

use crate::controller::response::ErrorResponse;
use rocket::response::Responder;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

/// A wrapper type to act as a reminder that the current string should be a
/// JSON object in string format.
pub type JsonString = String;

/// DTO representing a generic message to represent anything.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Responder)]
pub struct Message {
    /// The message content.
    pub message: JsonString,
}

impl Message {
    /// Serialize message to JSON string.
    pub fn json(&self) -> JsonString {
        serde_json::to_string(self).unwrap()
    }
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
        Self { message: msg }
    }
}

#[allow(unreachable_patterns)]
impl From<ErrorResponse> for Message {
    fn from(response: ErrorResponse) -> Self {
        match response {
            ErrorResponse::BadRequest(message) => Message { message },
            ErrorResponse::Unauthorized(message) => Message { message },
            ErrorResponse::NotFound(message) => Message { message },
            ErrorResponse::RequestTimeout(message) => Message { message },
            ErrorResponse::Conflict(message) => Message { message },
            ErrorResponse::PreconditionFailed(message) => Message { message },
            ErrorResponse::UnprocessableEntity(message) => Message { message },
            ErrorResponse::NoResponse(message) => Message { message },
            ErrorResponse::ClientClosedRequest(message) => Message { message },
            ErrorResponse::InternalServerError(message) => Message { message },
            ErrorResponse::NotImplemented(message) => Message { message },
            ErrorResponse::ServiceUnavailable(message) => Message { message },
            ErrorResponse::NetworkAuthenticationRequired(message) => Message { message },
            _ => unimplemented!(),
        }
    }
}
