extern crate bb8_diesel;
extern crate diesel;

use dotenv::dotenv;
use minerva_data::{db, encryption};
use minerva_rpc::users::users_server::UsersServer;
use std::env;
use tonic::transport::Server;

mod controller;
mod service;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: USERS service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();
    dotenv().ok();

    println!("Creating database connection pool...");
    let pool = db::make_connection_pool(15).await;

    let port = env::var("USERS_SERVICE_PORT").expect("Unable to read USERS_SERVICE_PORT");
    let addr = format!("0.0.0.0:{}", port).parse()?;
    encryption::init_hasher();

    println!("Starting USERS on {}...", addr);

    let server = Server::builder()
        .add_service(UsersServer::new(service::UsersService { pool }))
        .serve(addr);

    println!("USERS is ready to accept connections.");
    server.await?;
    println!("USERS shut down.");
    Ok(())
}
