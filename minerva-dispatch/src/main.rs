#![warn(clippy::all)]
#![warn(missing_docs)]

use minerva_broker as broker;
use minerva_cache as cache;
use minerva_data as data;

use broker::LapinPool;
use data::db::DBPool;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

mod controller;

#[tokio::main]
async fn main() {
    println!("Minerva System: DISPATCH");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");

    let mongoserver =
        env::var("MONGO_SERVICE_SERVER").expect("Unable to read MONGO_SERVICE_SERVER");

    let redisserver =
        env::var("REDIS_SERVICE_SERVER").expect("Unable to read REDIS_SERVICE_SERVER");

    let rmqserver =
        env::var("RABBITMQ_SERVICE_SERVER").expect("Unable to read RABBITMQ_SERVICE_SERVER");

    println!("Loading tenant configuration...");
    let tenant_config = data::tenancy::get_tenants("tenancy.toml");

    println!("Creating service connections...");
    let mongo_client = data::mongo::make_client(&mongoserver).await;
    let redis_client = cache::build_client(&redisserver).expect("Unable to create Redis client");
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

    println!("Starting listeners for each tenant...");
    let mut handlers = vec![];
    for t in tenants {
        let tenant = t.clone();
        let postgres = data::db::make_connection_pool(&tenant.clone(), &dbserver, 15).await;
        let rabbitmq = broker::make_connection_pool(&rmqserver, Some(&tenant), 15).await;
        let redis = redis_client.clone();
        let mongo = mongo_client.clone();

        handlers.push(tokio::spawn(async move {
            println!("Running queue listener for {}.", tenant);
            controller::queue_consume(tenant, rabbitmq, postgres, mongo.clone(), redis.clone())
                .await;
        }))
    }

    for handler in handlers {
        handler.await.unwrap();
    }
}
