use crate::service;
use dotenv::dotenv;
use futures_util::FutureExt;
use minerva_broker as broker;
use minerva_data::db;
use minerva_data::user as model;
use minerva_rpc::messages;
use minerva_rpc::metadata::ClientInterceptor;
use minerva_rpc::user::user_client::UserClient;
use minerva_rpc::user::user_server::UserServer;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, Server};
use tonic::Request;

async fn make_test_server(
    port: u32,
) -> (
    JoinHandle<()>,
    UserClient<InterceptedService<Channel, ClientInterceptor>>,
    oneshot::Sender<()>,
) {
    dotenv().ok();

    // Generate server address and client endpoint
    let address = format!("0.0.0.0:{}", port).parse().unwrap();
    let endpoint = format!("http://127.0.0.1:{}", port);
    let dbserver =
        env::var("DATABASE_SERVICE_SERVER").expect("Unable to read DATABASE_SERVICE_SERVER");
    let rmqserver =
        env::var("RABBITMQ_SERVICE_SERVER").expect("Unable to read RABBITMQ_SERVICE_SERVER");

    // Create database connection pool with a single connection
    let mut pools = HashMap::new();
    pools.insert(
        "minerva".into(),
        (
            db::make_connection_pool("minerva", &dbserver, 1).await,
            broker::make_connection_pool(&rmqserver, Some("minerva"), 1).await,
        ),
    );

    // Create single-time channel for shutdown signal passing
    let (tx, rx) = oneshot::channel();

    // Spawn server on a concurrent task
    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(UserServer::new(service::UserService { pools }))
            .serve_with_shutdown(address, rx.map(drop))
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = minerva_rpc::user::make_client(endpoint, "minerva".into(), "tester".into())
        .await
        .expect("Successfull connection to USER gRPC client");

    (handle, client, tx)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_index() {
    let (handle, mut client, tx) = make_test_server(10010).await;

    // Request list of all users, then print on success
    let index = Some(0);
    let response = client
        .index(Request::new(minerva_rpc::messages::PageIndex { index }))
        .await
        .unwrap();
    println!(
        "INDEX: {:#?}",
        minerva_data::user::message_to_vec(response.into_inner())
    );

    // Shutdown server
    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_show() {
    let (handle, mut client, tx) = make_test_server(10011).await;

    // Request a single invalid user, then print on success
    let index = 0;
    let response = client
        .show(Request::new(minerva_rpc::messages::EntityIndex { index }))
        .await;
    assert!(response.is_err());
    println!("SHOW (should error): {:#?}", response.unwrap_err());

    // Shutdown server
    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_store() {
    let (handle, mut client, tx) = make_test_server(10012).await;

    // Create a single user
    let response = client
        .store(messages::User {
            id: None,
            login: "fulano1234".to_string(),
            name: "Fulano de Tal1234".to_string(),
            email: None,
            password: Some("minhasenha123".to_string()),
        })
        .await
        .unwrap();

    let stored_user: model::User = response.into_inner().into();
    println!("STORE: {:#?}", stored_user);
    assert_eq!(stored_user.login, "fulano1234");
    assert_eq!(stored_user.name, "Fulano de Tal1234");
    assert_eq!(stored_user.pwhash, Vec::<u8>::new());

    // Remove that user
    let index = stored_user.id;
    let _ = client
        .delete(messages::EntityIndex { index })
        .await
        .unwrap();

    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_store_update_show() {
    let (handle, mut client, tx) = make_test_server(10013).await;

    // Create a single user
    let response = client
        .store(messages::User {
            id: None,
            login: "ciclano1234".to_string(),
            name: "Ciclano de Tal1234".to_string(),
            email: Some("ciclano1234@exemplo.com".to_string()),
            password: Some("outrasenha456".to_string()),
        })
        .await
        .unwrap();

    let stored_user: model::User = response.into_inner().into();
    assert_eq!(stored_user.login, "ciclano1234");
    assert_eq!(stored_user.name, "Ciclano de Tal1234");
    assert_eq!(stored_user.email.unwrap(), "ciclano1234@exemplo.com");
    assert_eq!(stored_user.pwhash, Vec::<u8>::new());

    // Update the given user
    let response = client
        .update(messages::User {
            id: Some(stored_user.id),
            login: "ciclano1234".to_string(),
            name: "Ciclano da Silva1234".to_string(),
            email: Some("ciclano1234@servidor.com".to_string()),
            password: None,
        })
        .await
        .unwrap();

    let stored_user: model::User = response.into_inner().into();
    println!("UPDATE: {:#?}", stored_user);
    assert_eq!(stored_user.login, "ciclano1234");
    assert_eq!(stored_user.name, "Ciclano da Silva1234");
    assert_eq!(stored_user.email.unwrap(), "ciclano1234@servidor.com");
    assert_eq!(stored_user.pwhash, Vec::<u8>::new());

    // Fetch the updated user yet again
    let index = stored_user.id;
    let response = client
        .show(Request::new(minerva_rpc::messages::EntityIndex { index }))
        .await
        .unwrap();

    let stored_user: model::User = response.into_inner().into();
    assert_eq!(stored_user.login, "ciclano1234");
    assert_eq!(stored_user.name, "Ciclano da Silva1234");
    assert_eq!(stored_user.email.unwrap(), "ciclano1234@servidor.com");
    assert_eq!(stored_user.pwhash, Vec::<u8>::new());

    let index = stored_user.id;
    let _ = client
        .delete(messages::EntityIndex { index })
        .await
        .unwrap();

    tx.send(()).unwrap();
    handle.await.unwrap();
}
