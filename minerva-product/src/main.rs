use dotenv::dotenv;
use minerva_data::encryption;
use minerva_rpc::products::products_server::ProductsServer;
use std::env;
use tonic::transport::Server;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: PRODUCTS service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();
    let port = env::var("PRODUCTS_SERVICE_PORT").expect("Unable to read PRODUCTS_SERVICE_PORT");
    let addr = format!("0.0.0.0:{}", port).parse()?;
    encryption::init_hasher();

    println!("Starting PRODUCTS on {}...", addr);

    let server = Server::builder()
        .add_service(ProductsServer::new(service::ProductsService::default()))
        .serve(addr);

    println!("PRODUCTS is ready to accept connections.");
    server.await?;
    println!("PRODUCTS shut down.");
    Ok(())
}
