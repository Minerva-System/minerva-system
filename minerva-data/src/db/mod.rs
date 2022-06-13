//! This module contains functions and structures related to handling the
//! PostgreSQL database, specially regarding connections and database creation.

use bb8::Pool;
use bb8_diesel::DieselConnection;
use bb8_diesel::DieselConnectionManager;
use diesel::{Connection, PgConnection};

mod create;

/// Type representing a Diesel connection to a database.
pub type DBConnection = DieselConnection<PgConnection>;

/// Type representing a Diesel connection pool to a database.
pub type DBPool = Pool<DieselConnectionManager<PgConnection>>;

pub use create::create_database;

/// Generates a string to connect to PostgreSQL RDBMS, given a tenant name and
/// the database server endpoint (e.g. `localhost:5432`). This function
/// assumes a user `postgres` with a password `postgres`.
///
/// # TODO
/// The default configuration for user and password could open a security
/// hole if not accessing an RDBMS spawned using Docker Compose or Kubernetes.
/// Consider changing this on the future.
pub fn build_database_string(tenant: &str, server: &str) -> String {
    format!("postgres://postgres:postgres@{}/{}", server, tenant)
}

/// Attempts to generate a single connection to the PostgreSQL RDBMS, without
/// error checks. This could be used to perform connections and evaluate if they
/// could actually be established.
///
/// This function needs the tenant name, which is equal to the database name,
/// and the server endpoint for the database (e.g. `localhost:5432`).
pub fn try_make_single_connection(
    tenant: &str,
    server: &str,
) -> Result<PgConnection, diesel::ConnectionError> {
    let url = build_database_string(tenant, server);
    PgConnection::establish(&url)
}

/// Creates a single connection to the PostgreSQL RDBMS, and panics if the
/// connection could not be established.
///
/// This function needs the tenant name, which is equal to the database name,
/// and the server endpoint for the database (e.g. `localhost:5432`).
pub fn make_single_connection(tenant: &str, server: &str) -> PgConnection {
    try_make_single_connection(tenant, server)
        .map_err(|e| panic!("Error establishing relational database connection: {}", e))
        .unwrap()
}

/// Creates a connection pool to the PostgreSQL RDBMS, given a maximum number of
/// connections, and panics if the connections could not be established.
///
/// This function needs the tenant name, which is equal to the database name,
/// and the server endpoint for the database (e.g. `localhost:5432`).
pub async fn make_connection_pool(tenant: &str, server: &str, max_connections: u32) -> DBPool {
    let url = build_database_string(tenant, server);

    let manager = DieselConnectionManager::<PgConnection>::new(&url);

    Pool::builder()
        .max_size(max_connections)
        .build(manager)
        .await
        .map_err(|e| panic!("Error creating database connection pool: {}", e))
        .unwrap()
}
