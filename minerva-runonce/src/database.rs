use minerva_data::db;
use minerva_data::{
    encryption,
    syslog::NewLog,
    user::{NewUser, User},
};
use std::env;

pub fn create_database(tenant: &str) {
    db::create_database(tenant)
        .map_err(|e| panic!("{}: Error while creating database: {}", tenant, e))
        .unwrap();
}

pub fn run_migrations(tenant: &str) {
    let connection = db::make_single_connection(tenant);
    println!("{}: Running pending database migrations...", tenant);
    diesel_migrations::run_pending_migrations(&connection)
        .map_err(|e| panic!("{}: Error while running database migrations: {}", tenant, e))
        .unwrap();
    println!("Migrations ran successfully.");
}

pub fn create_admin_user(tenant: &str) {
    use diesel::prelude::*;
    use minerva_data::schema::syslog;
    use minerva_data::schema::user::{self, dsl::*};

    println!("{}: Creating user for Administrator...", tenant);

    let connection = db::make_single_connection(tenant);

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

    let pw = env::var("ADMIN_PASSWORD").unwrap_or(String::from("admin"));

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
                    operation: 0,
                    datetime: chrono::offset::Utc::now(),
                    description: Some(format!("Add administrator ID {}", result.id)),
                })
                .execute(&connection)?;

            Ok(())
        })
        .map_err(|e| panic!("{}: Error registering user \"admin\": {}", tenant, e))
        .unwrap();
}
