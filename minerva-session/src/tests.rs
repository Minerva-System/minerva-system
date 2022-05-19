use crate::service;
use dotenv::dotenv;
use futures_util::FutureExt;
use minerva_data::{
    db, mongo,
    session::{NewSession, Session},
};
use minerva_rpc::{
    messages::SessionToken,
    metadata::ClientInterceptor,
    session::{session_client::SessionClient, session_server::SessionServer},
};
use mongodb::bson::{doc, oid::ObjectId, Document};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tonic::{
    codegen::InterceptedService,
    transport::{Channel, Server},
    Code, Request,
};

async fn make_test_server(
    port: u32,
) -> (
    JoinHandle<()>,
    SessionClient<InterceptedService<Channel, ClientInterceptor>>,
    oneshot::Sender<()>,
) {
    dotenv().ok();
    let address = format!("0.0.0.0:{}", port).parse().unwrap();
    let endpoint = format!("http://127.0.0.1:{}", port);
    let dbserver = env::var("DATABASE_SERVICE_SERVER").unwrap();
    let mongoserver = env::var("MONGO_SERVICE_SERVER").unwrap();

    let mut pools = HashMap::new();
    pools.insert(
        "minerva".into(),
        (
            db::make_connection_pool("minerva", &dbserver, 1).await,
            mongo::make_client(&mongoserver).await,
        ),
    );

    let (tx, rx) = oneshot::channel();

    let handle = tokio::spawn(async move {
        Server::builder()
            .add_service(SessionServer::new(service::SessionService { pools }))
            .serve_with_shutdown(address, rx.map(drop))
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let client =
        minerva_rpc::session::make_client(endpoint, "minerva".into(), "tester".into()).await;

    (handle, client, tx)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_single_session() {
    let (handle, mut client, tx) = make_test_server(11010).await;

    // Create session
    let new_session = NewSession {
        tenant: "minerva".to_string(),
        login: "admin".to_string(),
        password: "admin".to_string(),
    };

    let token = client
        .generate(Request::new(new_session.clone().into()))
        .await
        .map_err(|e| panic!("Error while requesting session generation: {}", e))
        .unwrap()
        .into_inner();

    println!("Token: {:#?}", token.token);

    // Retrieve data and evaluate
    let response = client
        .retrieve(Request::new(token.clone()))
        .await
        .map_err(|e| panic!("Error while requesting session data: {}", e))
        .unwrap();
    let session_data: Session = response.into_inner().into();
    assert_eq!(new_session.tenant, session_data.tenant);
    assert_eq!(new_session.login, session_data.login);

    // Logoff
    client
        .remove(Request::new(token))
        .await
        .map_err(|e| panic!("Error while requesting session termination: {}", e))
        .unwrap();

    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_multiple_sessions() {
    let (handle, mut client, tx) = make_test_server(11011).await;
    let mongoserver = env::var("MONGO_SERVICE_SERVER").unwrap();
    let mongo_client = mongo::make_client(&mongoserver).await;

    // Create session
    let new_session = NewSession {
        tenant: "minerva".to_string(),
        login: "admin".to_string(),
        password: "admin".to_string(),
    };

    let token = client
        .generate(Request::new(new_session.clone().into()))
        .await
        .map_err(|e| panic!("Error while requesting session generation: {}", e))
        .unwrap()
        .into_inner();

    // Check token
    let object_id = ObjectId::parse_str(
        String::from_utf8(
            base64::decode(token.token.as_bytes()).expect("Unable to decode as base64"),
        )
        .unwrap(),
    )
    .expect("Unable to parse ObjectId");

    let collection = mongo_client
        .database("minerva")
        .collection::<Document>("session");

    let object = collection
        .find_one(doc! { "_id": object_id }, None)
        .await
        .expect("Error searching for Session object")
        .expect("Unable to find Session object");

    println!("Token: {:#?}", token.token);
    println!("Session object: {:#?}", object);

    // Logoff
    client
        .remove(Request::new(token))
        .await
        .map_err(|e| panic!("Error while requesting session termination: {}", e))
        .unwrap();

    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_deleted_session() {
    let (handle, mut client, tx) = make_test_server(11012).await;

    // Create session
    let new_session = NewSession {
        tenant: "minerva".to_string(),
        login: "admin".to_string(),
        password: "admin".to_string(),
    };
    let token = client
        .generate(Request::new(new_session.clone().into()))
        .await
        .map_err(|e| panic!("Error while requesting session generation: {}", e))
        .unwrap()
        .into_inner();

    // Logoff
    client
        .remove(Request::new(token.clone()))
        .await
        .map_err(|e| panic!("Error while requesting session termination: {}", e))
        .unwrap();

    // Retrieve deleted session
    match client.retrieve(Request::new(token)).await {
        Ok(session) => panic!("Session was not deleted: {:#?}", session.into_inner()),
        Err(status) => assert_eq!(status.code(), Code::NotFound),
    }

    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_invalid_session() {
    let (handle, mut client, tx) = make_test_server(11013).await;

    // Retrieve invalid session
    let token = "NjI4NmE4YjQ2YTY1OGYzN2Y1ZGUwYzU1".to_string();
    match client.retrieve(Request::new(SessionToken { token })).await {
        Ok(session) => panic!("Session was not deleted: {:#?}", session.into_inner()),
        Err(status) => assert_eq!(status.code(), Code::NotFound),
    }

    tx.send(()).unwrap();
    handle.await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn integration_test_session_token() {
    let (handle, mut client, tx) = make_test_server(11014).await;

    // Create session
    let new_session = NewSession {
        tenant: "minerva".to_string(),
        login: "admin".to_string(),
        password: "admin".to_string(),
    };

    let token1 = client
        .generate(Request::new(new_session.clone().into()))
        .await
        .map_err(|e| panic!("Error while requesting session #1 generation: {}", e))
        .unwrap()
        .into_inner();

    let token2 = client
        .generate(Request::new(new_session.clone().into()))
        .await
        .map_err(|e| panic!("Error while requesting session #2 generation: {}", e))
        .unwrap()
        .into_inner();

    println!("Token 1: {:#?}", token1.token);
    println!("Token 2: {:#?}", token2.token);

    // Retrieve data and evaluate
    let response1 = client
        .retrieve(Request::new(token1.clone()))
        .await
        .map_err(|e| panic!("Error while requesting session #1 data: {}", e))
        .unwrap();
    let session1_data: Session = response1.into_inner().into();
    assert_eq!(new_session.tenant, session1_data.tenant);
    assert_eq!(new_session.login, session1_data.login);

    let response2 = client
        .retrieve(Request::new(token2.clone()))
        .await
        .map_err(|e| panic!("Error while requesting session #2 data: {}", e))
        .unwrap();
    let session2_data: Session = response2.into_inner().into();
    assert_eq!(new_session.tenant, session2_data.tenant);
    assert_eq!(new_session.login, session2_data.login);

    // Logoff
    client
        .remove(Request::new(token1))
        .await
        .map_err(|e| panic!("Error while requesting session #1 termination: {}", e))
        .unwrap();

    client
        .remove(Request::new(token2))
        .await
        .map_err(|e| panic!("Error while requesting session #2 termination: {}", e))
        .unwrap();

    tx.send(()).unwrap();
    handle.await.unwrap();
}
