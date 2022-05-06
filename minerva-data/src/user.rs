use crate::encryption;
use crate::schema::user;
use minerva_rpc::messages;

#[derive(Queryable, Serialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub pwhash: Vec<u8>,
}

#[derive(Insertable, Default, Debug)]
#[table_name = "user"]
pub struct NewUser {
    pub login: String,
    pub name: String,
    pub email: Option<String>,
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
                .unwrap_or(vec![]),
        }
    }
}

// Test: Convert message to user

impl Into<messages::User> for User {
    fn into(self) -> messages::User {
        messages::User {
            id: Some(self.id),
            login: self.login.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            password: None,
        }
    }
}

// Test: Convert user to message

// Test: Back-and-forth conversions for message and user

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

// Test: Convert message to insertable user

pub fn message_to_vec(message: messages::UserList) -> Vec<User> {
    message.users.iter().map(|u| u.clone().into()).collect()
}

// Test: Conversions from user list message to users

pub fn vec_to_message(v: Vec<User>) -> messages::UserList {
    messages::UserList {
        users: v.iter().map(|u| u.clone().into()).collect(),
    }
}

// Test: Conversions from users to user list message
