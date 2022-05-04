use minerva_data::db;
use minerva_data::{
    encryption,
    user::{InsertableUser, User},
};
use std::env;

pub fn run_migrations() {
    let connection = db::make_single_connection();
    println!("Running pending database migrations...");
    diesel_migrations::run_pending_migrations(&connection)
        .map_err(|e| panic!("Error while running database migrations: {}", e))
        .unwrap();
    println!("Migrations ran successfully.");
}

pub fn create_admin_user() {
    use diesel::prelude::*;
    use minerva_data::schema::user::{self, dsl::*};

    println!("Creating user for Administrator...");

    let connection = db::make_single_connection();

    if user
        .filter(login.eq("admin"))
        .first::<User>(&connection)
        .optional()
        .map_err(|e| panic!("Error fetching \"admin\" user: {}", e))
        .unwrap()
        .is_some()
    {
        println!("Administrator is already registered.");
        return;
    }

    let pw = env::var("ADMIN_PASSWORD").unwrap_or(String::from("admin"));

    let admin_data = InsertableUser {
        login: "admin".to_string(),
        name: "Administrator".to_string(),
        email: None,
        pwhash: encryption::generate_hash(&pw),
    };

    diesel::insert_into(user::table)
        .values(&admin_data)
        .get_result::<User>(&connection)
        .map_err(|e| panic!("Error registering user \"admin\": {}", e))
        .unwrap();
}
