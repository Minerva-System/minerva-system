use bb8::Pool;
use bb8_diesel::DieselConnection;
use bb8_diesel::DieselConnectionManager;
use diesel::{Connection, PgConnection};
use std::env;

pub type DBConnection = DieselConnection<PgConnection>;
pub type DBPool = Pool<DieselConnectionManager<PgConnection>>;

pub fn make_single_connection() -> PgConnection {
    let url = env::var("DATABASE_URL")
        .map_err(|e| panic!("Error reading database URL: {}", e))
        .unwrap();

    PgConnection::establish(&url)
        .map_err(|e| panic!("Error establishing database connection: {}", e))
        .unwrap()
}

pub async fn make_connection_pool(max_connections: u32) -> DBPool {
    let url = env::var("DATABASE_URL")
        .map_err(|e| panic!("Error reading database URL: {}", e))
        .unwrap();

    let manager = DieselConnectionManager::<PgConnection>::new(&url);

    Pool::builder()
        .max_size(max_connections)
        .build(manager)
        .await
        .map_err(|e| panic!("Error creating database connection pool: {}", e))
        .unwrap()
}
