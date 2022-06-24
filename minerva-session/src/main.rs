//! # Minerva System: SESSION Service
//!
//! ## About this service
//! This service's responsibility is to provide a means for managing user
//! authentication, especially user sessions. Through this module, an existing
//! user can start a session on the system so that it becomes possible to
//! perform certain other operations.
//!
//! This service does not manage user data, but should have direct access to
//! both databases so that it can check authentication data and create the
//! actual session data, which is stored and managed server-side.

#![warn(clippy::all)]
#![warn(missing_docs)]

use dotenv::dotenv;
use minerva_data::{db, mongo};
use minerva_rpc::session::session_server::SessionServer;
use std::collections::HashMap;
use std::env;
use tonic::transport::Server;

mod repository;
mod service;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: SESSION service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    let port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");
    let mongoserver =
        env::var("MONGO_SERVICE_SERVER").expect("Unable to read MONGO_SERVICE_SERVER");

    let mut pools = HashMap::new();
    let mongo = mongo::make_client(&mongoserver).await;
    for tenant in minerva_data::tenancy::get_tenants("tenancy.toml") {
        let pool = db::make_connection_pool(&tenant.database, &dbserver, tenant.connections).await;
        pools.insert(tenant.database.clone(), (pool, mongo.clone()));
        println!(
            "Added database connections for tenant {} ({} connections + 1 MongoDB client).",
            tenant.name, tenant.connections
        );
    }

    let addr = format!("0.0.0.0:{}", port).parse()?;

    println!("Starting SESSION on {}...", addr);

    let server = Server::builder()
        .add_service(SessionServer::new(service::SessionService { pools }))
        .serve(addr);

    println!("SESSION is ready to accept connections.");
    server.await?;
    println!("SESSION shut down.");
    Ok(())
}
