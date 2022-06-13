//! # Minerva System: PRODUCT Service
//!
//! ## About this service
//! This service's responsibility is that of managing anything related to
//! products, namely creation, updating, removal and listing. This also relates
//! to business rules related to these operations.
//!
//! This service is also not responsible for managing the stock of any products.
//! For that, refer to the STOCK Service.

#![warn(clippy::all)]
#![warn(missing_docs)]

use dotenv::dotenv;
use minerva_data::encryption;
use minerva_rpc::products::products_server::ProductsServer;
use std::env;
use tonic::transport::Server;

mod service;

/// Entry point for this service.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: PRODUCT service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();
    let port = env::var("PRODUCT_SERVICE_PORT").expect("Unable to read PRODUCT_SERVICE_PORT");
    let addr = format!("0.0.0.0:{}", port).parse()?;
    encryption::init_hasher();

    println!("Starting PRODUCT on {}...", addr);

    let server = Server::builder()
        .add_service(ProductsServer::new(service::ProductsService::default()))
        .serve(addr);

    println!("PRODUCT is ready to accept connections.");
    server.await?;
    println!("PRODUCT shut down.");
    Ok(())
}
