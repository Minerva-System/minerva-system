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
pub struct InsertableUser {
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub pwhash: Vec<u8>,
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

impl From<messages::User> for InsertableUser {
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

pub fn message_to_vec(message: messages::UserList) -> Vec<User> {
    message.users.iter().map(|u| u.clone().into()).collect()
}

pub fn vec_to_message(v: Vec<User>) -> messages::UserList {
    messages::UserList {
        users: v.iter().map(|u| u.clone().into()).collect(),
    }
}
