//! This module contains Data Transfer Objects for the `user` table, which
//! represents the registered users for this tenancy.

use crate::encryption;
use crate::schema::user;
use minerva_rpc::messages;

/// DTO representing a single entry on the `user` table.
#[derive(Queryable, Serialize, Clone, Debug)]
pub struct User {
    /// ID of the entry on the table.
    pub id: i32,
    /// Username of the user for this entry.
    pub login: String,
    /// Full name of the user for this entry.
    pub name: String,
    /// Optional e-mail of the user for this entry.
    pub email: Option<String>,
    /// Hash of the password of the user for this entry.
    #[serde(skip_serializing)]
    pub pwhash: Vec<u8>,
}

/// DTO representing user data received through a REST request.
#[derive(Serialize, Deserialize, Default)]
pub struct RecvUser {
    /// Username of the user being emplaced.
    pub login: String,
    /// Full name of the user being emplaced.
    pub name: String,
    /// Optional e-mail of the user being emplaced.
    /// Shouldn't clash with any other e-mail on the table.
    pub email: Option<String>,
    /// Password of the user being emplaced.
    /// If updating a user, using `None` will prevent updating the password.
    pub password: Option<String>,
}

/// DTO representing a new entry on the `user` table.
#[derive(Insertable, Default, Debug)]
#[table_name = "user"]
pub struct NewUser {
    /// Username of the user being created.
    pub login: String,
    /// Full name of the user being created.
    pub name: String,
    /// Optional e-mail of the user being created.
    /// Shouldn't clash with any other e-mail on the table.
    pub email: Option<String>,
    /// Hash of the password of the user being created.
    pub pwhash: Vec<u8>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        (self.id == other.id)
            && (self.login == other.login)
            && (self.name == other.name)
            && (self.email == other.email)
    }
}

impl PartialEq for NewUser {
    fn eq(&self, other: &Self) -> bool {
        (self.login == other.login) && (self.name == other.name) && (self.email == other.email)
    }
}

impl From<messages::User> for User {
    fn from(message: messages::User) -> Self {
        Self {
            id: message.id.expect("Cannot retrieve ID for user"),
            login: message.login.clone(),
            name: message.name.clone(),
            email: message.email.clone(),
            pwhash: message
                .password
                .map(|pw| encryption::generate_hash(&pw))
                .unwrap_or_default(),
        }
    }
}

impl Into<messages::User> for User {
    fn into(self) -> messages::User {
        messages::User {
            id: Some(self.id),
            login: self.login.clone(),
            name: self.name.clone(),
            email: self.email,
            password: None,
        }
    }
}

impl Into<messages::User> for RecvUser {
    fn into(self) -> messages::User {
        messages::User {
            id: None,
            login: self.login.trim().to_string(),
            name: self.name.trim().to_string(),
            email: self.email.map(|e| e.trim().to_string()),
            password: if let Some(pw) = self.password {
                let pw = pw.trim().to_string();
                if pw.is_empty() {
                    None
                } else {
                    Some(pw)
                }
            } else {
                None
            },
        }
    }
}

impl From<messages::User> for NewUser {
    fn from(message: messages::User) -> Self {
        Self {
            login: message.login.clone(),
            name: message.name.clone(),
            email: message.email.clone(),
            pwhash: message
                .password
                .map(|pw| encryption::generate_hash(&pw))
                .unwrap(),
        }
    }
}

/// Converts a `UserList` message into a `Vec` of `User` DTOs so that it
/// can be sent back as a REST response.
pub fn message_to_vec(message: messages::UserList) -> Vec<User> {
    message.users.iter().map(|u| u.clone().into()).collect()
}

/// Converts a `Vec` of `User` DTOs to a `UserList` so that it can be
/// sent back as a gRPC response.
pub fn vec_to_message(v: Vec<User>) -> messages::UserList {
    messages::UserList {
        users: v.iter().map(|u| u.clone().into()).collect(),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::encryption;

    #[test]
    fn convert_message_to_user() {
        encryption::init_hasher();

        let user = User {
            id: 0,
            login: "teste".into(),
            name: "Fulano da Silva".into(),
            email: Some("fulano@exemplo.com".into()),
            pwhash: encryption::generate_hash("senha"),
        };

        let msg = messages::User {
            id: Some(0),
            login: "teste".into(),
            name: "Fulano da Silva".into(),
            email: Some("fulano@exemplo.com".into()),
            password: Some("senha".into()),
        };

        let msg_user: User = msg.into();

        assert_eq!(user, msg_user);
        assert!(encryption::check_hash("senha", &user.pwhash));
        assert!(encryption::check_hash("senha", &msg_user.pwhash));
    }

    #[test]
    fn convert_user_to_message() {
        encryption::init_hasher();

        let msg = messages::User {
            id: Some(0),
            login: "ciclano".into(),
            name: "Ciclano da Silva".into(),
            email: Some("ciclano@exemplo.com".into()),
            password: Some("senha123".into()),
        };

        let expected = User {
            id: 0,
            login: "ciclano".into(),
            name: "Ciclano da Silva".into(),
            email: Some("ciclano@exemplo.com".into()),
            pwhash: encryption::generate_hash("senha123"),
        };

        let expected_msg: messages::User = expected.clone().into();

        assert_eq!(msg.id, expected_msg.id);
        assert_eq!(msg.login, expected_msg.login);
        assert_eq!(msg.name, expected_msg.name);
        assert_eq!(msg.email, expected_msg.email);
        assert!(encryption::check_hash("senha123", &expected.pwhash));
        assert!(expected_msg.password.is_none());
    }

    #[test]
    fn convert_message_to_newuser() {
        encryption::init_hasher();

        let newuser = NewUser {
            login: "teste".into(),
            name: "Fulano da Silva".into(),
            email: Some("fulano@exemplo.com".into()),
            pwhash: encryption::generate_hash("senha"),
        };

        let msg = messages::User {
            id: Some(0),
            login: "teste".into(),
            name: "Fulano da Silva".into(),
            email: Some("fulano@exemplo.com".into()),
            password: Some("senha".into()),
        };

        let msg_user: NewUser = msg.into();

        assert_eq!(newuser, msg_user);
        assert!(encryption::check_hash("senha", &newuser.pwhash));
        assert!(encryption::check_hash("senha", &msg_user.pwhash));
    }

    #[test]
    fn convert_recvuser_to_message() {
        // Insert case
        let expected = messages::User {
            id: None,
            login: "ciclano".into(),
            name: "Ciclano da Silva".into(),
            email: Some("ciclano@exemplo.com".into()),
            password: Some("senha123".into()),
        };

        let user = RecvUser {
            login: "ciclano".into(),
            name: "Ciclano da Silva".into(),
            email: Some("ciclano@exemplo.com".into()),
            password: Some("senha123".into()),
        };

        let msg: messages::User = user.into();
        assert_eq!(expected, msg);

        // Update case -- No password
        let expected = messages::User {
            id: Some(2),
            login: "ciclano".into(),
            name: "Ciclano da Silva".into(),
            email: Some("ciclano@exemplo.com".into()),
            password: None,
        };

        let user = RecvUser {
            login: "ciclano".into(),
            name: "Ciclano da Silva".into(),
            email: Some("ciclano@exemplo.com".into()),
            password: None,
        };

        let mut msg: messages::User = user.into();
        msg.id = Some(2);
        assert_eq!(expected, msg);

        // Any case: Trim strings
        let user = RecvUser {
            login: "    ciclano      ".into(),
            name: "      Ciclano da Silva    ".into(),
            email: Some("  ciclano@exemplo.com    ".into()),
            password: Some("    senha123          ".into()),
        };
        let msg: messages::User = user.into();

        assert_eq!(msg.login, "ciclano");
        assert_eq!(msg.name, "Ciclano da Silva");
        assert_eq!(msg.email.unwrap(), "ciclano@exemplo.com");
        assert_eq!(msg.password.unwrap(), "senha123");
    }

    #[test]
    fn convert_userlist_message_to_user_vec() {
        encryption::init_hasher();

        let mut msg = messages::UserList {
            users: vec![
                messages::User {
                    id: Some(1),
                    login: "fulano".into(),
                    name: "Fulano da Silva".into(),
                    email: Some("fulano@exemplo.com".into()),
                    password: Some("senha1234".into()),
                },
                messages::User {
                    id: Some(2),
                    login: "ciclano".into(),
                    name: "Ciclano da Silva".into(),
                    email: Some("ciclano@exemplo.com".into()),
                    password: Some("senha5678".into()),
                },
                messages::User {
                    id: Some(3),
                    login: "beltrano".into(),
                    name: "Beltrano da Silva".into(),
                    email: Some("beltrano@exemplo.com".into()),
                    password: Some("senha9821".into()),
                },
            ],
        };

        let expected: Vec<User> = vec![
            User {
                id: 1,
                login: "fulano".into(),
                name: "Fulano da Silva".into(),
                email: Some("fulano@exemplo.com".into()),
                pwhash: encryption::generate_hash("senha1234"),
            },
            User {
                id: 2,
                login: "ciclano".into(),
                name: "Ciclano da Silva".into(),
                email: Some("ciclano@exemplo.com".into()),
                pwhash: encryption::generate_hash("senha5678"),
            },
            User {
                id: 3,
                login: "beltrano".into(),
                name: "Beltrano da Silva".into(),
                email: Some("beltrano@exemplo.com".into()),
                pwhash: encryption::generate_hash("senha9821"),
            },
        ];

        for it in expected
            .iter()
            .zip(message_to_vec(msg.clone()).iter_mut())
            .zip(msg.users.iter_mut())
        {
            let ((expected, converted), message) = it;
            assert_eq!(converted, expected);
            assert!(encryption::check_hash(
                message.password.as_ref().unwrap(),
                &expected.pwhash
            ));
            assert!(encryption::check_hash(
                message.password.as_ref().unwrap(),
                &converted.pwhash
            ));
        }
    }

    #[test]
    fn convert_user_vec_to_userlist_message() {
        encryption::init_hasher();

        let users: Vec<User> = vec![
            User {
                id: 1,
                login: "fulano".into(),
                name: "Fulano da Silva".into(),
                email: Some("fulano@exemplo.com".into()),
                pwhash: encryption::generate_hash("senha1234"),
            },
            User {
                id: 2,
                login: "ciclano".into(),
                name: "Ciclano da Silva".into(),
                email: Some("ciclano@exemplo.com".into()),
                pwhash: encryption::generate_hash("senha5678"),
            },
            User {
                id: 3,
                login: "beltrano".into(),
                name: "Beltrano da Silva".into(),
                email: Some("beltrano@exemplo.com".into()),
                pwhash: encryption::generate_hash("senha9821"),
            },
        ];

        let expected = messages::UserList {
            users: vec![
                messages::User {
                    id: Some(1),
                    login: "fulano".into(),
                    name: "Fulano da Silva".into(),
                    email: Some("fulano@exemplo.com".into()),
                    password: None,
                },
                messages::User {
                    id: Some(2),
                    login: "ciclano".into(),
                    name: "Ciclano da Silva".into(),
                    email: Some("ciclano@exemplo.com".into()),
                    password: None,
                },
                messages::User {
                    id: Some(3),
                    login: "beltrano".into(),
                    name: "Beltrano da Silva".into(),
                    email: Some("beltrano@exemplo.com".into()),
                    password: None,
                },
            ],
        };

        for it in expected
            .users
            .iter()
            .zip(vec_to_message(users).users.iter_mut())
        {
            let (expected, converted) = it;
            assert_eq!(expected, converted);
            assert!(converted.password.is_none());
        }
    }
}
