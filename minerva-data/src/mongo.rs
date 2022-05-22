//! This module contains functions and structures related to handling the
//! MongoDB database, specially regarding connections, database and collection
//! creation and management.

use mongodb::{options::ClientOptions, Client};

/// Default user session duration for the `session` collection, in seconds.
/// Calculates to one week by default. Minimum value is one minute, as per
/// limitations of MongoDB related to expiration checks.
pub static SESSION_DURATION: u64 = 60 * 60 * 24 * 7;

/// Generates a string to connect to MongoDB, given a server endpoint (e.g.
/// `localhost:27017`). This function assumes that MongoDB was configured with
/// the credentials of a user `root` with password `mongo`.
///
/// # TODO
/// The default configuration for user and password could open a security
/// hole if not accessing a database spawned using Docker Compose or Kubernetes.
/// Consider changing this on the future.
pub fn build_client_string(server: &str) -> String {
    format!("mongodb://root:mongo@{}/admin", server)
}

/// Attempts to generate a single client for the MongoDB sevice, without error
/// checks. This could be used to perform connections and evaluate if they could
/// actually be estalished.
///
/// This function needs the server endpoint for the MongoDB service.
pub async fn try_make_client(server: &str) -> Result<Client, mongodb::error::Error> {
    let options = ClientOptions::parse(build_client_string(server)).await?;
    Client::with_options(options)
}

/// Creates a client connection to the MongoDB service, and panics if the
/// connection could not be established.
///
/// This function needs the server endpoint for the MongoDB service.
pub async fn make_client(server: &str) -> Client {
    try_make_client(server)
        .await
        .map_err(|e| {
            panic!(
                "Error establishing non-relational database connection: {}",
                e
            )
        })
        .unwrap()
}
