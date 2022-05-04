use crate::schema::user;

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
