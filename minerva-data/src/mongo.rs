use mongodb::{options::ClientOptions, Client};

pub static SESSION_DURATION: u64 = 60 * 60 * 24 * 7;

pub fn build_client_string(server: &str) -> String {
    format!("mongodb://root:mongo@{}:27017/admin", server)
}

pub async fn try_make_client(server: &str) -> Result<Client, mongodb::error::Error> {
    let options = ClientOptions::parse(build_client_string(server)).await?;
    Client::with_options(options)
}

pub async fn make_client(server: &str) -> Client {
    try_make_client(server)
        .await
        .map_err(|e| {
            panic!(
                "Error establishing non-relational database connection: {}",
                e
            )
        })
        .unwrap()
}
