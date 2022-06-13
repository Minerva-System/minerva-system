//! # Minerva System: REPORT Service
//!
//! ## About this service
//! This service's responsibility is that of generating reports from different
//! kinds of data that shall be fed to it. In exchange, the service gives back
//! a report that could be rendered as a document format, depending on the
//! requestor's demand.
//!
//! This service is not responsible for processing or recovering data from the
//! database. Any extra calculations on the report data should depend only on
//! the data in question that was fed to it.

#![warn(clippy::all)]
#![warn(missing_docs)]

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: REPORT service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();
    let _port = env::var("REPORT_SERVICE_PORT").expect("Unable to read REPORT_SERVICE_PORT");

    Ok(())
}
