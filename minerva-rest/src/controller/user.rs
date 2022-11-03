//! This submodule describes routes for managing the data for a `User` entity,
//! particularly with respect to connecting to the `USER` gRPC service.

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

/// Returns the list of routes for this module.
pub fn routes() -> Vec<Route> {
    routes![index, show, store, update, delete]
}

/// Retrieves the endpoint for the gRPC users service. Requires that the proper
/// environment variables are defined.
pub fn get_endpoint() -> String {
    let port = env::var("USER_SERVICE_PORT").expect("Unable to read USER_SERVICE_PORT");
    let srv = env::var("USER_SERVICE_SERVER").expect("Unable to read USER_SERVICE_SERVER");
    format!("http://{}:{}", srv, port)
}

/// Route for listing all users.
///
/// This route uses the concept of pages, starting with page index `0`. The
/// page number should be passed as a request parameter through the URL, under
/// a value named `page`. If omitted, it is assumed to be `0`.
///
/// Upon success, returns a list of users in JSON format, containing up to the
/// number of users per page as defined in the `USER` microservice.
///
/// # Request examples
///
/// ```bash
/// curl -X GET 'http://localhost:9000/minerva/user' \
///      -H 'Authorization: Bearer {token}'
///
/// curl -X GET 'http://localhost:9000/minerva/user?page=0' \
///      -H 'Authorization: Bearer {token}'
/// ```
#[get("/<_>/user?<page>")]
async fn index(session: SessionInfo, page: Option<i64>) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::INDEX > USER::INDEX @ {}", endpoint),
    );

    let client = rpc::user::make_client(endpoint, tenant, requestor).await;
    if client.is_err() {
        return Response::generate_error(client);
    }
    let mut client = client.unwrap();

    let response = client
        .index(Request::new(rpc::messages::PageIndex { index: page }))
        .await
        .map(|msg| data::user::message_to_vec(msg.into_inner()));
    Response::respond(response)
}

/// Route for fetching data of a single user.
///
/// The numeric user ID should be passed through the route.
///
/// Upon success, retrieves data for a single user of the given ID in JSON
/// format.
///
/// # Request example
///
/// ```bash
/// curl -X GET 'http://localhost:9000/user/1' \
///      -H 'Authorization: Bearer {token}'
/// ```
#[get("/<_>/user/<id>")]
async fn show(session: SessionInfo, id: i32) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::SHOW > USER::SHOW @ {}", endpoint),
    );

    let client = rpc::user::make_client(endpoint, tenant, requestor).await;
    if client.is_err() {
        return Response::generate_error(client);
    }
    let mut client = client.unwrap();

    let index = id;
    let response: Result<data::user::User, tonic::Status> = client
        .show(Request::new(rpc::messages::EntityIndex { index }))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// Route for creating a new user.
///
/// To use this route, use a POST request, sending as body a JSON containing the
/// expected data for creating a new user.
///
/// Upon success, returns the data for the created user as if it were requested
/// through the `show` method.
///
/// # Request example
///
/// ```bash
/// curl -X POST 'http://localhost:9000/user' \
///      -H 'Content-Type: application/json' \
///      -H 'Authorization: Bearer {token}'
///      -d '{"login": "fulano", "name": "Fulano da Silva", "email": null, "password": "senha123"}'
/// ```
#[post("/<_>/user", data = "<body>")]
async fn store(session: SessionInfo, body: Json<data::user::RecvUser>) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    let message: rpc::messages::User = body.0.into();

    if message.login == *"unknown" {
        return Response::BadRequest(
            json!({ "message": "Username \"unknown\" is reserved" }).to_string(),
        );
    }

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::STORE > USER::STORE @ {}", endpoint),
    );

    let client = rpc::user::make_client(endpoint, tenant, requestor).await;
    if client.is_err() {
        return Response::generate_error(client);
    }
    let mut client = client.unwrap();

    let response: Result<data::user::User, tonic::Status> = client
        .store(Request::new(message))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// Route for updating data for a user.
///
/// To use this route, use a PUT request. The ID of the user to be updated
/// should also be passed through the URL.
///
/// Ignore `password` or pass it as an empty string if you wish to prevent
/// password updates.
///
/// Upon success, returns the data for the created user as if it were requested
/// through the `show` method.
///
/// # Request example
///
/// ```bash
/// curl -X PUT 'http://localhost:9000/user/2' \
///      -H 'Content-Type: application/json'  \
///      -H 'Authorization: Bearer {token}'
///      -d '{"login": "fulano", "name": "Fulano da Silva", "email": null, "password": null}'
/// ```
#[put("/<_>/user/<id>", data = "<body>")]
async fn update(session: SessionInfo, id: i32, body: Json<data::user::RecvUser>) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::UPDATE > USER::UPDATE @ {}", endpoint),
    );

    let client = rpc::user::make_client(endpoint, tenant, requestor).await;
    if client.is_err() {
        return Response::generate_error(client);
    }
    let mut client = client.unwrap();

    let mut message: rpc::messages::User = body.0.into();
    message.id = Some(id);
    let response: Result<data::user::User, tonic::Status> = client
        .update(Request::new(message))
        .await
        .map(|msg| msg.into_inner().into());
    Response::respond(response)
}

/// Route for removing a user altogether.
///
/// To use this route, use a DELETE request. The ID of the user to be updated
/// should also be passed through the URL.
///
/// Upon success, returns an empty object.
///
/// # Request example
///
/// ```bash
/// curl -X DELETE 'http://localhost:9000/user/2' \
///      -H 'Authorization: Bearer {token}'
/// ```
#[delete("/<_>/user/<index>")]
async fn delete(session: SessionInfo, index: i32) -> Response {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::DELETE > USER::DELETE @ {}", endpoint),
    );

    let client = rpc::user::make_client(endpoint, tenant, requestor).await;
    if client.is_err() {
        return Response::generate_error(client);
    }
    let mut client = client.unwrap();

    let response = client
        .delete(Request::new(rpc::messages::EntityIndex { index }))
        .await;
    Response::respond_empty(response)
}
