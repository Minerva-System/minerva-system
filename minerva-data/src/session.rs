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

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct NewSession {
    pub tenant: String,
    pub login: String,
    pub password: String,
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

impl From<messages::SessionCreationData> for NewSession {
    fn from(msg: messages::SessionCreationData) -> Self {
        Self {
            tenant: msg.tenant.trim().to_string(),
            login: msg.login.trim().to_string(),
            password: msg.password.trim().to_string(),
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

impl Into<messages::SessionCreationData> for NewSession {
    fn into(self) -> messages::SessionCreationData {
        messages::SessionCreationData {
            tenant: self.tenant.trim().to_string(),
            login: self.login.trim().to_string(),
            password: self.password.trim().to_string(),
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

    #[test]
    fn convert_session_creation_data_to_creation_message() {
        let data = NewSession {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            password: "admin".to_string(),
        };

        let expected = messages::SessionCreationData {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            password: "admin".to_string(),
        };

        let result: messages::SessionCreationData = data.into();

        assert_eq!(expected, result);
    }

    #[test]
    fn convert_session_creation_message_to_creation_data() {
        let msg = messages::SessionCreationData {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            password: "admin".to_string(),
        };

        let expected = NewSession {
            tenant: "minerva".to_string(),
            login: "admin".to_string(),
            password: "admin".to_string(),
        };

        let result: NewSession = msg.into();

        assert_eq!(expected, result);
    }
}
