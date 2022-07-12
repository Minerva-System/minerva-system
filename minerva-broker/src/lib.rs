//! This is a library for working with routines related to the message broker,
//! or more specifically, messages sent and received to/from RabbitMQ.

#![warn(clippy::all)]
#![warn(missing_docs)]

use bb8_lapin::{bb8::Pool, LapinConnectionManager};
use lapin::{Connection, ConnectionProperties, Error};

/// Default user for authentication on RabbitMQ.
const AUTH_USER: &str = "rabbitmq";

/// Default password for authentication on RabbitMQ.
const AUTH_PASS: Option<&str> = Some("minerva");

/// Names of default queues that should be present on RabbitMQ, for each tenant.
pub const QUEUES: &'static [&str] = &["session_management"];

pub mod model;

/// Represents a pool of connections to RabbitMQ.
pub type LapinPool = Pool<LapinConnectionManager>;

/// Generate connection URL to a vhost configuration API.
fn make_vhost_url(host: &str, vhost: &str) -> String {
    format!("http://{}:15672/api/vhosts/{}", host, vhost)
}

/// Builds an AMQP protocol URI for RabbitMQ.
/// If `vhost` is an empty string, assumes that the connection string should
/// be to the default virtual host.
pub fn build_broker_uri(host: &str, vhost: &str) -> String {
    format!(
        "amqp://{}:{}@{}:5672/{}",
        AUTH_USER,
        AUTH_PASS.unwrap(),
        host,
        if vhost == "" { "%2f" } else { vhost }
    )
}

/// Attempts to create a single connection to RabbitMQ. If the `vhost` is set
/// to `None`, assumes that the connection should be established to the default
/// virtual host.
pub async fn make_connection(host: &str, vhost: Option<&str>) -> Result<Connection, Error> {
    let uri = build_broker_uri(host, vhost.unwrap_or(""));
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);
    Connection::connect(&uri, options).await
}

/// Creates a connection pool to RabbitMQ. If the `vhost` is set to `None`,
/// assumes that the connection should be established to the default virtual
/// host.
pub async fn make_connection_pool(
    host: &str,
    vhost: Option<&str>,
    max_connections: u32,
) -> LapinPool {
    let uri = build_broker_uri(host, vhost.unwrap_or(""));
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);
    let manager = LapinConnectionManager::new(&uri, options);
    Pool::builder()
        .max_size(max_connections)
        .build(manager)
        .await
        .map_err(|e| panic!("Error creating RabbitMQ connection pool: {}", e))
        .unwrap()
}

/// Checks whether RabbitMQ can be accessed or not.
pub async fn check_virtual_host(host: &str) -> Result<bool, reqwest::Error> {
    let url = make_vhost_url(host, "");
    let client = reqwest::Client::new();
    // Returns 200 when could fetch the API
    Ok(client
        .get(url)
        .basic_auth(AUTH_USER, AUTH_PASS)
        .send()
        .await?
        .status()
        .as_u16()
        == 200)
}

/// Creates a virtual host on RabbitMQ.
/// Virtual hosts are created by performing a PUT request on the management
/// API. If the request does not return an error, returns an optional boolean.
/// This result should be `None` if the request was unsuccessful, or `true`
/// or `false` depending on whether the virtual host was created or not.
/// This value is `true` if the request results in a 201 status, or `false` if
/// the result had a status of 204.
pub async fn make_virtual_host(host: &str, vhost: &str) -> Result<Option<bool>, reqwest::Error> {
    let url = make_vhost_url(host, vhost);
    let client = reqwest::Client::new();
    // Returns 201 on creation or 204 when it already exists
    match client
        .put(url)
        .basic_auth(AUTH_USER, AUTH_PASS)
        .send()
        .await
    {
        Ok(response) => Ok(if !response.status().is_success() {
            None
        } else {
            Some(response.status().as_u16() == 201)
        }),
        Err(e) => Err(e),
    }
}
