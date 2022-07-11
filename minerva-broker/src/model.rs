use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionMessage {
    Remove { user: String },
}

impl SessionMessage {
    pub fn from(json: String) -> Self {
        serde_json::from_str(&json).expect("Unable to generate message from string")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to generate JSON for session message")
    }
}
