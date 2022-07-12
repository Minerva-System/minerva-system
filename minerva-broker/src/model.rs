//! This submodule describes models for messages that can be enqueued on
//! RabbitMQ. Their default queueing format is serialized as JSON strings.

use serde::{Deserialize, Serialize};

/// Represents a message for the `session_management` queue, more specifically
/// related to managing user sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionMessage {
    /// Message for removing all sessions of a given user.
    Remove {
        /// The user whose sessions should be removed.
        user: String,
    },
}

impl SessionMessage {
    /// Reconstructs a message from the given JSON string.
    pub fn from(json: String) -> Self {
        serde_json::from_str(&json).expect("Unable to generate message from string")
    }

    /// Serializes this message to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to generate JSON for session message")
    }
}
