#[macro_use]
extern crate diesel_migrations;
extern crate minerva_data;

use dotenv::dotenv;

embed_migrations!();

mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: RUNONCE");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    println!("Running database preparation...");
    database::run_migrations();
    database::create_admin_user();

    println!("All runs were successful.");
    println!("RUNONCE shut down.");
    Ok(())
}
