use bb8::Pool;
use bb8_diesel::DieselConnection;
use bb8_diesel::DieselConnectionManager;
use diesel::{Connection, PgConnection};

mod create;

pub type DBConnection = DieselConnection<PgConnection>;
pub type DBPool = Pool<DieselConnectionManager<PgConnection>>;

pub use create::create_database;

pub fn build_database_string(tenant: &str) -> String {
    format!("postgres://postgres:postgres@localhost/{}", tenant)
}

pub fn try_make_single_connection(tenant: &str) -> Result<PgConnection, diesel::ConnectionError> {
    let url = build_database_string(tenant);
    PgConnection::establish(&url)
}

pub fn make_single_connection(tenant: &str) -> PgConnection {
    try_make_single_connection(tenant)
        .map_err(|e| panic!("Error establishing database connection: {}", e))
        .unwrap()
}

pub async fn make_connection_pool(tenant: &str, max_connections: u32) -> DBPool {
    let url = build_database_string(tenant);

    let manager = DieselConnectionManager::<PgConnection>::new(&url);

    Pool::builder()
        .max_size(max_connections)
        .build(manager)
        .await
        .map_err(|e| panic!("Error creating database connection pool: {}", e))
        .unwrap()
}
