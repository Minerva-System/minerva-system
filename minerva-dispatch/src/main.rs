//! # Minerva System: DISPATCH Service
//!
//! ## About this service
//! This service is responsible for dispatching messages from the message broker
//! (RabbitMQ) to other parts of the system, and sometimes interacting with other
//! third-party services as well.
//!
//! Though this service is not created with the intention of managing data
//! directly, that may still happen, though it is not desired since specific
//! microservices may manage business rules in an intended way.
//!
//! ## Testing
//! This module currently has no unit nor integration tests. This is because
//! these kinds of tests are expensive and are not really needed for this
//! module, since these only perform a dispatch -- if it is being deployed
//! successfully, then it works. (I know this is not ideal, but hey, I need
//! to release this...)

#![warn(clippy::all)]
#![warn(missing_docs)]

use minerva_broker as broker;
use minerva_cache as cache;
use minerva_data as data;

use broker::LapinPool;
use data::db::DBPool;
use dotenv::dotenv;
use log::info;
use std::collections::HashMap;
use std::env;

mod controller;
mod error;

/// Entry point for this module.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: DISPATCH");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    let logconfig = env::var("LOG_CONFIG_FILE").unwrap_or_else(|_| "./logging.yml".to_owned());
    log4rs::init_file(logconfig, Default::default())?;

    let dbserver = env::var("DATABASE_SERVICE_SERVER")?;

    let mongoserver = env::var("MONGO_SERVICE_SERVER")?;

    let redisserver = env::var("REDIS_SERVICE_SERVER")?;

    let rmqserver = env::var("RABBITMQ_SERVICE_SERVER")?;

    info!("Loading tenant configuration...");
    let tenant_config = data::tenancy::get_tenants("tenancy.toml");

    info!("Creating service connections...");
    let mongo_client = data::mongo::make_client(&mongoserver).await;
    let redis_client = cache::build_client(&redisserver)?;
    let mut tenant_clients: HashMap<String, (DBPool, LapinPool)> = HashMap::new();

    for tenant in tenant_config.clone() {
        let dbpool =
            data::db::make_connection_pool(&tenant.database, &dbserver, tenant.connections).await;
        let lapinpool =
            broker::make_connection_pool(&rmqserver, Some(&tenant.database), tenant.connections)
                .await;
        tenant_clients.insert(tenant.database.clone(), (dbpool, lapinpool));
    }

    let tenants = tenant_config
        .iter()
        .map(|cfg| cfg.database.clone())
        .collect::<Vec<String>>();

    info!("Starting listeners for each tenant...");
    let mut handlers = vec![];
    for t in tenants {
        let tenant = t.clone();
        let postgres = data::db::make_connection_pool(&tenant.clone(), &dbserver, 15).await;
        let rabbitmq = broker::make_connection_pool(&rmqserver, Some(&tenant), 15).await;
        let redis = redis_client.clone();
        let mongo = mongo_client.clone();

        handlers.push(tokio::spawn(async move {
            info!("Running queue listener for {}.", tenant);
            controller::queue_consume(tenant, rabbitmq, postgres, mongo, redis)
                .await
                .unwrap();
        }))
    }

    for handler in handlers {
        handler.await?;
    }

    Ok(())
}
