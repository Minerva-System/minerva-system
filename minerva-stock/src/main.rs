//! # Minerva System: STOCK Service
//!
//! ## About this service
//! This service's responsibility is that of managing anything related to a
//! product's stock and its business rules.
//!
//! This service is not responsible for managing products themselves, but only
//! data related to the product's stock and how this information is stored on
//! the database.

#![warn(clippy::all)]
#![warn(missing_docs)]

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: STOCK service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();
    let _port = env::var("STOCK_SERVICE_PORT").expect("Unable to read STOCK_SERVICE_PORT");

    Ok(())
}
