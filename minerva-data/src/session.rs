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
            tenant: msg.tenant.trim().to_string(),
            login: msg.login.trim().to_string(),
            creation_date: DateTime::from_millis(msg.creation_date),
        }
    }
}

impl From<messages::SessionCreationData> for Session {
    fn from(msg: messages::SessionCreationData) -> Self {
        Self {
            tenant: msg.tenant.trim().to_string(),
            login: msg.login.trim().to_string(),
            creation_date: DateTime::now(),
        }
    }
}

impl Into<messages::SessionData> for Session {
    fn into(self) -> messages::SessionData {
        messages::SessionData {
            tenant: self.tenant.trim().to_string(),
            login: self.login.trim().to_string(),
            creation_date: self.creation_date.timestamp_millis(),
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use mongodb::bson::DateTime;

    #[test]
    fn convert_session_message_to_data() {
        let creation_date = DateTime::now();

        let message = messages::SessionData {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            creation_date: creation_date.timestamp_millis(),
        };

        let expected = Session {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            creation_date,
        };

        let result: Session = message.into();

        assert_eq!(expected, result);
    }

    #[test]
    fn convert_data_to_session_message() {
        let creation_date = DateTime::now();

        let data = Session {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            creation_date,
        };

        let expected = messages::SessionData {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            creation_date: creation_date.timestamp_millis(),
        };

        let result: messages::SessionData = data.into();

        assert_eq!(expected, result);
    }

    #[test]
    fn convert_session_creation_message_to_data() {
        let message = messages::SessionCreationData {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            password: String::new(),
        };

        let expected = Session {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            creation_date: DateTime::now(),
        };

        let result: Session = message.into();

        assert_eq!(expected, result);
    }
}
