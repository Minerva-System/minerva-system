use super::response;
use minerva_data as data;
use minerva_rpc as rpc;
use response::Response;
use rocket::serde::json::Json;
use rocket::Route;
use std::env;
use tonic::Request;

// TODO: Change this! This should come from authentication.
const REQUESTOR: &str = "admin";

pub fn routes() -> Vec<Route> {
    routes![index, show, store, update, delete]
}

fn get_endpoint() -> String {
    let port = env::var("USER_SERVICE_SERVER").expect("Unable to read USER_SERVICE_SERVER");
    let srv = env::var("USER_SERVICE_PORT").expect("Unable to read USER_SERVICE_PORT");
    format!("http://{}:{}", srv, port)
}

/// # Request example
///
/// ```bash
/// curl -X GET 'http://localhost:9000/teste/users'
/// curl -X GET 'http://localhost:9000/teste/users?page=0'
/// ```
#[get("/<tenant>/users?<page>")]
async fn index(tenant: String, page: Option<i64>) -> Response {
    let endpoint = get_endpoint();
    let mut client = rpc::users::make_client(endpoint, tenant, REQUESTOR.into()).await;
    let response = client
        .index(Request::new(rpc::messages::PageIndex { index: page }))
        .await
        .map(|msg| data::user::message_to_vec(msg.into_inner()));
    Response::respond(response)
}

/// # Request example
///
/// ```bash
/// curl -X GET 'http://localhost:9000/teste/users/1'
/// ```
#[get("/<tenant>/users/<id>")]
async fn show(tenant: String, id: i32) -> Response {
    let endpoint = get_endpoint();
    let mut client = rpc::users::make_client(endpoint, tenant, REQUESTOR.into()).await;
    let index = id;
    let response: Result<data::user::User, tonic::Status> = client
        .show(Request::new(rpc::messages::EntityIndex { index }))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// Request example
/// ```bash
/// curl -X POST 'http://localhost:9000/teste/users' \
///      -H 'Content-Type: application/json' \
///      -d '{"login": "fulano", "name": "Fulano da Silva", "email": null, "password": "senha123"}'
/// ```
#[post("/<tenant>/users", data = "<body>")]
async fn store(tenant: String, body: Json<data::user::RecvUser>) -> Response {
    let endpoint = get_endpoint();
    let mut client = rpc::users::make_client(endpoint, tenant, REQUESTOR.into()).await;
    let message = body.0.into();
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
/// curl -X PUT 'http://localhost:9000/teste/users/2' \
///      -H 'Content-Type: application/json' \
///      -d '{"login": "fulano", "name": "Fulano da Silva", "email": null, "password": null}'
/// ```
#[put("/<tenant>/users/<id>", data = "<body>")]
async fn update(tenant: String, id: i32, body: Json<data::user::RecvUser>) -> Response {
    let endpoint = get_endpoint();
    let mut client = rpc::users::make_client(endpoint, tenant, REQUESTOR.into()).await;
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
/// curl -X DELETE 'http://localhost:9000/teste/users/2'
/// ```
#[delete("/<tenant>/users/<index>")]
async fn delete(tenant: String, index: i32) -> Response {
    let endpoint = get_endpoint();
    let mut client = rpc::users::make_client(endpoint, tenant, REQUESTOR.into()).await;
    let response = client.delete(Request::new(
	rpc::messages::EntityIndex { index }
    )).await;
    Response::respond_empty(response)
}
