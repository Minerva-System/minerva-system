use diesel::{Connection, PgConnection};
use std::env;

pub fn make_single_connection() -> PgConnection {
    let url = env::var("DATABASE_URL")
        .map_err(|e| panic!("Error reading database URL: {}", e))
        .unwrap();

    PgConnection::establish(&url)
        .map_err(|e| panic!("Error establishing database connection: {}", e))
        .unwrap()
}
