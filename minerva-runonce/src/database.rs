//! This module wraps all functions related to operations that should be run
//! once on the database, when the entire system starts.

use minerva_data::db;
use minerva_data::{
    encryption,
    syslog::{NewLog, OpType},
    user::{NewUser, User},
};
use std::env;

/// Awaits for database availability on a spinlock.
pub async fn database_spinlock(server: &str) {
    let mut lock = true;
    while lock {
        let conn = db::try_make_single_connection("postgres", server);
        if conn.is_ok() {
            lock = false;
        } else {
            tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
        }
    }
}

/// Create a database for a specific tenant, if it doesn't exist.
/// Panics if unable to create database.
pub fn create_database(tenant: &str, server: &str) {
    db::create_database(tenant, server)
        .map_err(|e| panic!("{}: Error while creating database: {}", tenant, e))
        .unwrap();
}

/// Run pending database migrations for a specific tenant.
/// Panics if unable to run any migrations, if they weren't run already.
pub fn run_migrations(tenant: &str, server: &str) {
    let connection = db::make_single_connection(tenant, server);
    println!("{}: Running pending database migrations...", tenant);
    diesel_migrations::run_pending_migrations(&connection)
        .map_err(|e| panic!("{}: Error while running database migrations: {}", tenant, e))
        .unwrap();
    println!("Migrations ran successfully.");
}

/// Create an Administrator user (login "admin") for a specific tenant, if it
/// doesn't exist already.
///
/// The default administrator password can be configured through the environment
/// variable `ADMIN_PASSWORD`. Otherwise, defaults to "admin".
pub fn create_admin_user(tenant: &str, server: &str) {
    use diesel::prelude::*;
    use minerva_data::schema::syslog;
    use minerva_data::schema::user::{self, dsl::*};

    println!("{}: Creating user for Administrator...", tenant);

    let connection = db::make_single_connection(tenant, server);

    if user
        .filter(login.eq("admin"))
        .first::<User>(&connection)
        .optional()
        .map_err(|e| panic!("{}: Error fetching \"admin\" user: {}", tenant, e))
        .unwrap()
        .is_some()
    {
        println!("{}: Administrator is already registered.", tenant);
        return;
    }

    let pw = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| String::from("admin"));

    let admin_data = NewUser {
        login: "admin".to_string(),
        name: "Administrator".to_string(),
        email: None,
        pwhash: encryption::generate_hash(&pw),
    };

    connection
        .build_transaction()
        .read_write()
        .run::<(), diesel::result::Error, _>(|| {
            let result = diesel::insert_into(user::table)
                .values(&admin_data)
                .get_result::<User>(&connection)?;

            diesel::insert_into(syslog::table)
                .values(&NewLog {
                    service: "RUNONCE".to_string(),
                    requestor: "runonce".to_string(),
                    entity: "user".to_string(),
                    operation: OpType::Insert,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Add administrator ID {}", result.id)),
                })
                .execute(&connection)?;

            Ok(())
        })
        .map_err(|e| panic!("{}: Error registering user \"admin\": {}", tenant, e))
        .unwrap();
}
