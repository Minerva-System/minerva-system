//! This module wraps the repository which handles the user DTOs.

use diesel::prelude::*;
use diesel::result::Error;
use lapin::{options::BasicPublishOptions, BasicProperties};
use minerva_broker as broker;
use minerva_data::db::DBConnection;
use minerva_data::db::DBPool;
use minerva_data::syslog::{NewLog, OpType};
use minerva_data::user as model;

/// Default page size of a user list.
const USER_PAGE_SIZE: i64 = 20;

/// Grabs a list of users, paged. Expects a page number. If none or a negative
/// value is provided, returns page 0.
pub fn get_list(page: i64, connection: &DBConnection) -> Result<Vec<model::User>, Error> {
    use minerva_data::schema::user::dsl::*;
    let page = if page < 0 { 0 } else { page };
    let offset = page * USER_PAGE_SIZE;
    user.order(id)
        .limit(USER_PAGE_SIZE)
        .offset(offset)
        .load::<model::User>(connection)
}

/// Grabs a specific user, given its ID on the database.
pub fn get_user(user_id: i32, connection: &DBConnection) -> Result<Option<model::User>, Error> {
    use minerva_data::schema::user::dsl::*;
    user.filter(id.eq(user_id))
        .first::<model::User>(connection)
        .optional()
}

/// Creates a new user, for a given requestor, which shall also be a user.
pub fn add_user(
    data: model::NewUser,
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
                    service: "USER".to_string(),
                    requestor,
                    entity: "user".to_string(),
                    operation: OpType::Insert,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Add user ID {}", result.id)),
                })
                .execute(connection)?;

            Ok(result)
        })
}

/// Updates the data for a user, for a given requestor, which shall also be a user.
pub fn update_user(
    data: model::User,
    requestor: String,
    connection: &DBConnection,
) -> Result<model::User, Error> {
    use minerva_data::schema::syslog;
    use minerva_data::schema::user::dsl::*;

    let old = if let Some(value) = get_user(data.id, connection)? {
        value
    } else {
        return Err(Error::NotFound);
    };

    // Relies on model::User's Eq and/or PartialEq
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
                    service: "USER".to_string(),
                    requestor,
                    entity: "user".to_string(),
                    operation: OpType::Update,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Update user ID {}", result.id)),
                })
                .execute(connection)?;

            Ok(result)
        })
}

/// Deletes a user, for a given requestor, which shall also be a user.
pub async fn delete_user(
    user_id: i32,
    requestor: String,
    dbpool: &DBPool,
    rabbitmq: &lapin::Connection,
) -> Result<(), Error> {
    use minerva_data::schema::syslog;
    use minerva_data::schema::user::dsl::*;

    let connection = dbpool.get().await.unwrap();
    //.map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

    let result = connection
        .build_transaction()
        .read_write()
        .run::<model::User, Error, _>(|| {
            let target = user.filter(id.eq(user_id));

            let entity = target.get_result::<model::User>(&*connection)?;

            diesel::delete(target).execute(&*connection)?;

            diesel::insert_into(syslog::table)
                .values(&NewLog {
                    service: "USER".to_string(),
                    requestor,
                    entity: "user".to_string(),
                    operation: OpType::Delete,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Delete user ID {}", user_id)),
                })
                .execute(&*connection)?;

            Ok(entity)
        });

    // Queue message on RabbitMQ so that the session service
    // asynchronously deletes the user's sessions.
    // We can just inform the tenant and the login and that should be it
    let channel = rabbitmq
        .create_channel()
        .await
        .expect("Could not create RabbitMQ channel");

    let json = "Test Message!"; // TODO

    // TODO: Improve error handling here
    channel
        .basic_publish(
            "",
            "session_management",
            BasicPublishOptions::default(),
            json.as_bytes(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();

    Ok(())
}
