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
use log::{debug, info};
use minerva_cache as cache;
use minerva_data::{db, mongo};
use minerva_rpc::session::session_server::SessionServer;
use std::collections::HashMap;
use std::env;
use tonic::transport::Server;

mod repository;
mod service;

#[cfg(test)]
mod tests;

/// Entry point for this module.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: SESSION service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    let logconfig = env::var("LOG_CONFIG_FILE").unwrap_or_else(|_| "./logging.yml".to_owned());

    match log4rs::init_file(logconfig, Default::default()) {
        Ok(_) => info!("Log system initialized."),
        Err(e) => eprintln!(
            "Failure while initializing logs: {:?}\n\
			     You might be flying solo now.",
            e
        ),
    }

    let port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");
    let mongoserver =
        env::var("MONGO_SERVICE_SERVER").expect("Unable to read MONGO_SERVICE_SERVER");
    let redisserver =
        env::var("REDIS_SERVICE_SERVER").expect("Unable to read REDIS_SERVICE_SERVER");

    let mut pools = HashMap::new();
    let mongo = mongo::make_client(&mongoserver).await;
    let redis = cache::build_client(&redisserver).expect("Unable to create Redis client");
    for tenant in minerva_data::tenancy::get_tenants("tenancy.toml") {
        let pool = db::make_connection_pool(&tenant.database, &dbserver, tenant.connections).await;
        pools.insert(
            tenant.database.clone(),
            (pool, mongo.clone(), redis.clone()),
        );
        debug!(
            "Added database connections for tenant {} ({} connections + 1 MongoDB client + 1 Redis client).",
            tenant.name, tenant.connections
        );
    }

    let addr = format!("0.0.0.0:{}", port).parse()?;

    info!("Starting SESSION on {}...", addr);

    let server = Server::builder()
        .add_service(SessionServer::new(service::SessionService { pools }))
        .serve(addr);

    info!("SESSION is ready to accept connections.");
    server.await?;
    info!("SESSION shut down.");
    Ok(())
}
