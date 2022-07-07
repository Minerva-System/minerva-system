//! # Minerva System: RUNONCE Service
//!
//! ## About this service
//! This service's responsibility is that of preparing the Minerva System
//! environment for use, specially when it comes to preparing databases by
//! running migrations, for example.
//!
//! This service is best used as a job, and should be run only when deploying
//! new changes to the database service is really needed. This means that it
//! should be a single-run job or a cron job at best.

#![warn(clippy::all)]
#![warn(missing_docs)]

#[macro_use]
extern crate diesel_migrations;
extern crate minerva_data;

use dotenv::dotenv;
use std::env;

embed_migrations!();

mod database;
mod mongo;
mod rabbitmq;

/// Entry point for this service.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: RUNONCE");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");

    let mongoserver =
        env::var("MONGO_SERVICE_SERVER").expect("Unable to read MONGO_SERVICE_SERVER");

    println!("Await for relational database on spinlock...");
    database::database_spinlock(&dbserver);

    println!("Await for non-relational database on spinlock...");
    mongo::database_spinlock(&mongoserver).await;

    println!("Running preparation...");

    for tenant in minerva_data::tenancy::get_tenants("tenancy.toml") {
        println!("Running configuration for tenant \"{}\"...", tenant.name);
        database::create_database(&tenant.database, &dbserver);
        database::run_migrations(&tenant.database, &dbserver);
        database::create_admin_user(&tenant.database, &dbserver);
        mongo::prepare_database(&tenant.database, &mongoserver).await?;
    }

    println!("All runs were successful.");
    println!("RUNONCE shut down.");
    Ok(())
}
