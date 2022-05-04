use crate::encryption;
use crate::schema::user;
use minerva_rpc::messages;

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub pwhash: Vec<u8>,
}

#[derive(Insertable)]
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
            id: message.id.unwrap_or(0),
            login: message.login.clone(),
            name: message.name.clone(),
            email: message.email.clone(),
            pwhash: message
                .password
                .map(|pw| encryption::generate_hash(&pw))
                .expect("Cannot generate hashed password from message"),
        }
    }
}

impl From<messages::UserWrite> for InsertableUser {
    fn from(message: messages::UserWrite) -> Self {
        let message = message.user.unwrap();
        Self {
            login: message.login.clone(),
            name: message.name.clone(),
            email: message.email.clone(),
            pwhash: message
                .password
                .map(|pw| encryption::generate_hash(&pw))
                .expect("Cannot generate hashed password from message"),
        }
    }
}

pub fn msg_get_write_data(message: messages::UserWrite) -> (String, InsertableUser) {
    (message.required_by.clone(), message.into())
}

pub fn msg_get_list(message: messages::UserList) -> Vec<User> {
    message.users.iter().map(|u| u.clone().into()).collect()
}
