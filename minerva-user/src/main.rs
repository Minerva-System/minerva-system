//! # Minerva System: USERS Service
//!
//! ## About this service
//! This service's responsibility is that of managing anything related to users,
//! which are the primary actors for interfacing with the rest of the system.
//!
//! This service is not responsible for managing user authentication, for
//! example, which is handled elsewhere.

#![warn(clippy::all)]
#![warn(missing_docs)]

extern crate bb8_diesel;
extern crate diesel;

use dotenv::dotenv;
use minerva_data::{db, encryption};
use minerva_rpc::users::users_server::UsersServer;
use std::collections::HashMap;
use std::env;
use tonic::transport::Server;

mod repository;
mod service;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: USER service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();
    dotenv().ok();

    println!("Creating database connection pools...");

    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");

    let mut pools = HashMap::new();
    for tenant in minerva_data::tenancy::get_tenants("tenancy.toml") {
        pools.insert(
            tenant.database.clone(),
            db::make_connection_pool(&tenant.database, &dbserver, tenant.connections).await,
        );
        println!(
            "Added pool for tenant {} ({} connections).",
            tenant.name, tenant.connections
        );
    }

    let port = env::var("USER_SERVICE_PORT").expect("Unable to read USER_SERVICE_PORT");
    let addr = format!("0.0.0.0:{}", port).parse()?;
    encryption::init_hasher();

    println!("Starting USER on {}...", addr);

    let server = Server::builder()
        .add_service(UsersServer::new(service::UsersService { pools }))
        .serve(addr);

    println!("USER is ready to accept connections.");
    server.await?;
    println!("USER shut down.");
    Ok(())
}
