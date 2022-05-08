#[macro_use]
extern crate diesel_migrations;
extern crate minerva_data;

use dotenv::dotenv;
use std::env;

embed_migrations!();

mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: RUNONCE");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    println!("Running database preparation...");

    let dbserver = env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");

    for tenant in minerva_data::tenancy::get_tenants("tenancy.toml") {
        println!("Running configuration for tenant \"{}\"...", tenant.name);
        database::create_database(&tenant.database, &dbserver);
        database::run_migrations(&tenant.database, &dbserver);
        database::create_admin_user(&tenant.database, &dbserver);
    }

    println!("All runs were successful.");
    println!("RUNONCE shut down.");
    Ok(())
}
