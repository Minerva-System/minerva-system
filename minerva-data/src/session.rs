use minerva_rpc::messages;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub tenant: String,
    pub login: String,
    pub creation_date: DateTime,
}

impl From<messages::SessionData> for Session {
    fn from(msg: messages::SessionData) -> Self {
        Self {
            tenant: msg.tenant,
            login: msg.login,
            creation_date: DateTime::now(),
        }
    }
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn convert_message_to_data() {
        todo!();
    }

    #[test]
    fn convert_data_to_message() {
        todo!();
    }
}
