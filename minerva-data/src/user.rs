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

pub fn message_to_vec(message: messages::UserList) -> Vec<User> {
    message.users.iter().map(|u| u.clone().into()).collect()
}

pub fn vec_to_message(v: Vec<User>) -> messages::UserList {
    messages::UserList {
        users: v.iter().map(|u| u.clone().into()).collect(),
    }
}

#[test]
fn convert_message_to_user() {
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
fn message_user_conversions() {
    // Convert stuff back-and-forth and add assertions based on that.
    // Remember the rule of never converting a password hash back to a message
    todo!();
}

#[test]
fn convert_message_to_newuser() {
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
fn convert_userlist_message_to_user_vec() {
    // Same as before, but converting a list of user messages to a vec of users
    todo!();
}

#[test]
fn convert_user_vec_to_userlist_message() {
    // Same as before, but converting a vec of users to a list of user messages
    todo!();
}
