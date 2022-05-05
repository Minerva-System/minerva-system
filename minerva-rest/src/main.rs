use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: REST service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();
    let _port = env::var("REST_SERVICE_PORT").expect("Unable to read REST_SERVICE_PORT");

    Ok(())
}
