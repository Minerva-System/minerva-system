use super::response;
use crate::fairings::auth::SessionInfo;
use crate::utils;
use minerva_data as data;
use minerva_rpc as rpc;
use response::Response;
use rocket::serde::json::Json;
use rocket::Route;
use serde_json::json;
use std::env;
use tonic::Request;

pub fn routes() -> Vec<Route> {
    routes![index, show, store, update, delete]
}

pub fn get_endpoint() -> String {
    let port = env::var("USER_SERVICE_PORT").expect("Unable to read USER_SERVICE_PORT");
    let srv = env::var("USER_SERVICE_SERVER").expect("Unable to read USER_SERVICE_SERVER");
    format!("http://{}:{}", srv, port)
}

/// # Request example
///
/// ```bash
/// curl -X GET 'http://localhost:9000/users'
/// curl -X GET 'http://localhost:9000/users?page=0'
/// ```
#[get("/users?<page>")]
async fn index(session: SessionInfo, page: Option<i64>) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::INDEX > USERS::INDEX @ {}", endpoint),
    );

    let mut client = rpc::users::make_client(endpoint, tenant, requestor).await;
    let response = client
        .index(Request::new(rpc::messages::PageIndex { index: page }))
        .await
        .map(|msg| data::user::message_to_vec(msg.into_inner()));
    Response::respond(response)
}

/// # Request example
///
/// ```bash
/// curl -X GET 'http://localhost:9000/users/1'
/// ```
#[get("/users/<id>")]
async fn show(session: SessionInfo, id: i32) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::SHOW > USERS::SHOW @ {}", endpoint),
    );

    let mut client = rpc::users::make_client(endpoint, tenant, requestor).await;
    let index = id;
    let response: Result<data::user::User, tonic::Status> = client
        .show(Request::new(rpc::messages::EntityIndex { index }))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// Request example
/// ```bash
/// curl -X POST 'http://localhost:9000/users' \
///      -H 'Content-Type: application/json' \
///      -d '{"login": "fulano", "name": "Fulano da Silva", "email": null, "password": "senha123"}'
/// ```
#[post("/users", data = "<body>")]
async fn store(session: SessionInfo, body: Json<data::user::RecvUser>) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    let message: rpc::messages::User = body.0.into();

    if message.login == "unknown".to_string() {
        return Response::BadRequest(
            json!({ "message": "Username \"unknown\" is reserved" }).to_string(),
        );
    }

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::STORE > USERS::STORE @ {}", endpoint),
    );

    let mut client = rpc::users::make_client(endpoint, tenant, requestor).await;
    let response: Result<data::user::User, tonic::Status> = client
        .store(Request::new(message))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// # Request example
///
/// Ignore `password` or pass it as an empty string if you wish to prevent updates.
///
/// ```bash
/// curl -X PUT 'http://localhost:9000/users/2' \
///      -H 'Content-Type: application/json' \
///      -d '{"login": "fulano", "name": "Fulano da Silva", "email": null, "password": null}'
/// ```
#[put("/users/<id>", data = "<body>")]
async fn update(session: SessionInfo, id: i32, body: Json<data::user::RecvUser>) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::UPDATE > USERS::UPDATE @ {}", endpoint),
    );

    let mut client = rpc::users::make_client(endpoint, tenant, requestor).await;
    let mut message: rpc::messages::User = body.0.into();
    message.id = Some(id);
    let response: Result<data::user::User, tonic::Status> = client
        .update(Request::new(message))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// # Request example
///
/// ```bash
/// curl -X DELETE 'http://localhost:9000/users/2'
/// ```
#[delete("/users/<index>")]
async fn delete(session: SessionInfo, index: i32) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::DELETE > USERS::DELETE @ {}", endpoint),
    );

    let mut client = rpc::users::make_client(endpoint, tenant, requestor).await;
    let response = client
        .delete(Request::new(rpc::messages::EntityIndex { index }))
        .await;
    Response::respond_empty(response)
}
