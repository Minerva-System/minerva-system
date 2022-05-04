use crate::service;
use dotenv::dotenv;
use futures_util::FutureExt;
use minerva_data::db;
use minerva_rpc::users::users_client::UsersClient;
use minerva_rpc::users::users_server::UsersServer;
use std::time::Duration;
use tokio::sync::oneshot;
use tonic::transport::{Endpoint, Server};
use tonic::Request;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_index() {
    dotenv().ok();
    let server_addr = format!("0.0.0.0:9010").parse().unwrap();
    let client_addr = Endpoint::from_static("http://127.0.0.1:9010");

    let pool = db::make_connection_pool(1).await;
    let (_tx, rx): (oneshot::Sender<Request<()>>, oneshot::Receiver<_>) = oneshot::channel();

    let server = tokio::spawn(async move {
        println!("Starting server...");
        let server = Server::builder()
            .add_service(UsersServer::new(service::UsersService { pool }))
            .serve_with_shutdown(server_addr, rx.map(drop));
        println!("Server is up.");
        server.await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut client = UsersClient::connect(client_addr).await.unwrap();
    let response = client.index(Request::new(())).await.unwrap();
    println!(
        "Response: {:?}",
        minerva_data::user::message_to_vec(response.into_inner()) // cargo test -- --nocapture
    );
}
