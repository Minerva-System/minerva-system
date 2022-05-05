use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Minerva System: SESSION service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();
    let _port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");

    Ok(())
}
