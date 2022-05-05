use diesel::prelude::*;
use diesel::result::Error;
use minerva_data::db::DBConnection;
use minerva_data::user as model;

const USER_PAGE_SIZE: i64 = 20;

pub fn get_list(page: i64, connection: &DBConnection) -> Result<Vec<model::User>, Error> {
    use minerva_data::schema::user::dsl::*;
    let offset = (page * USER_PAGE_SIZE) + 1;
    user.order(id)
        .limit(USER_PAGE_SIZE)
        .offset(offset)
        .load::<model::User>(connection)
}

pub fn get_user(user_id: i32, connection: &DBConnection) -> Result<Option<model::User>, Error> {
    use minerva_data::schema::user::dsl::*;
    user.filter(id.eq(user_id))
        .first::<model::User>(connection)
        .optional()
}

pub fn add_user(
    data: model::InsertableUser,
    connection: &DBConnection,
) -> Result<model::User, Error> {
    use minerva_data::schema::user;
    diesel::insert_into(user::table)
        .values(&data)
        .get_result::<model::User>(connection)
}

pub fn update_user(
    data: model::InsertableUser,
    connection: &DBConnection,
) -> Result<model::User, Error> {
    unimplemented!();
}

pub fn delete_user(user_id: i32, connection: &DBConnection) -> Result<(), Error> {
    unimplemented!();
}
