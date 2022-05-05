use diesel::prelude::*;
use diesel::result::Error;
use minerva_data::db::DBConnection;
use minerva_data::syslog::NewLog;
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
    requestor: String,
    connection: &DBConnection,
) -> Result<model::User, Error> {
    use minerva_data::schema::syslog;
    use minerva_data::schema::user;

    connection
        .build_transaction()
        .read_write()
        .run::<model::User, Error, _>(|| {
            let result = diesel::insert_into(user::table)
                .values(&data)
                .get_result::<model::User>(connection)?;

            diesel::insert_into(syslog::table)
                .values(&NewLog {
                    service: "USERS".to_string(),
                    requestor,
                    entity: "user".to_string(),
                    operation: 0,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Add user ID {}", result.id)),
                })
                .execute(connection)?;

            Ok(result)
        })
}

pub fn update_user(
    data: model::User,
    requestor: String,
    connection: &DBConnection,
) -> Result<model::User, Error> {
    use minerva_data::schema::syslog;
    use minerva_data::schema::user::dsl::*;

    let old = get_user(data.id, connection)?;

    let old = if old.is_none() {
        return Err(Error::NotFound);
    } else {
        old.unwrap()
    };

    if old == data {
        return Ok(old);
    }

    connection
        .build_transaction()
        .read_write()
        .run::<model::User, Error, _>(|| {
            let target = user.filter(id.eq(data.id));

            let result = diesel::update(target)
                .set((
                    login.eq(data.login),
                    name.eq(data.name),
                    email.eq(data.email),
                ))
                .get_result::<model::User>(connection)?;

            let result = if data.pwhash.is_empty() {
                result
            } else {
                diesel::update(target)
                    .set(pwhash.eq(data.pwhash))
                    .get_result::<model::User>(connection)?
            };

            diesel::insert_into(syslog::table)
                .values(&NewLog {
                    service: "USERS".to_string(),
                    requestor,
                    entity: "user".to_string(),
                    operation: 1,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Update user ID {}", result.id)),
                })
                .execute(connection)?;

            Ok(result)
        })
}

pub fn delete_user(
    user_id: i32,
    requestor: String,
    connection: &DBConnection,
) -> Result<(), Error> {
    use minerva_data::schema::syslog;
    use minerva_data::schema::user::dsl::*;

    connection
        .build_transaction()
        .read_write()
        .run::<(), Error, _>(|| {
            let target = user.filter(id.eq(user_id));

            diesel::delete(target).execute(connection)?;

            diesel::insert_into(syslog::table)
                .values(&NewLog {
                    service: "USERS".to_string(),
                    requestor,
                    entity: "user".to_string(),
                    operation: 2,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Delete user ID {}", user_id)),
                })
                .execute(connection)?;

            Ok(())
        })
}
