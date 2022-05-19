use dotenv::dotenv;
use minerva_data::{db, mongo};
use minerva_rpc::session::session_server::SessionServer;
use std::collections::HashMap;
use std::env;
use tonic::transport::Server;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: SESSION service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();
    dotenv().ok();

    let port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");

    let mut pools = HashMap::new();
    for tenant in minerva_data::tenancy::get_tenants("tenancy.toml") {
        let mongo = mongo::make_client(&tenant.database).await;
        let pool = db::make_connection_pool(&tenant.database, &dbserver, tenant.connections).await;
        pools.insert(tenant.database.clone(), (pool, mongo));
        println!(
            "Added database connections for tenant {} ({} connections + 1 MongoDB client).",
            tenant.name, tenant.connections
        );
    }

    let addr = format!("0.0.0.0:{}", port).parse()?;
    let server = Server::builder()
        .add_service(SessionServer::new(service::SessionService { pools }))
        .serve(addr);

    println!("SESSION is ready to accept connections.");
    server.await?;
    println!("SESSION shut down.");
    Ok(())
}
