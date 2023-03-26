//! This submodule describes routes for managing the data for a `User` entity,
//! particularly with respect to connecting to the `USER` gRPC service.

use super::response;
use crate::fairings::auth::SessionInfo;
use crate::utils;
use log::info;
use minerva_data as data;
use minerva_rpc as rpc;
use response::{ErrorResponse, RestResult};
use rocket::serde::json::Json;
use rocket::Route;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, openapi_get_routes_spec};
use std::env;
use tonic::Request;

/// Returns a tuple containing a vec of routes for this module, plus a structure
/// containing the OpenAPI specification for these routes.
pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![index, show, store, update, delete]
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
#[allow(unused_variables)]
#[openapi(tag = "User")]
#[get("/<tenant>/user?<page>")]
async fn index(
    tenant: String,
    session: SessionInfo,
    page: Option<i64>,
) -> RestResult<Vec<data::user::User>> {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("GET /user: request USER.index ({})", endpoint),
        )
    );

    let mut client = rpc::user::make_client(endpoint, tenant, requestor)
        .await
        .map_err(ErrorResponse::from)?;

    client
        .index(Request::new(rpc::messages::PageIndex { index: page }))
        .await
        .map(|msg| Json(data::user::message_to_vec(msg.into_inner())))
        .map_err(ErrorResponse::from)
}

/// Route for fetching data of a single user.
///
/// The numeric user ID should be passed through the route.
///
/// Upon success, retrieves data for a single user of the given ID in JSON
/// format.
#[allow(unused_variables)]
#[openapi(tag = "User")]
#[get("/<tenant>/user/<id>")]
async fn show(tenant: String, session: SessionInfo, id: i32) -> RestResult<data::user::User> {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("GET /user/<id>: request USER.show ({})", endpoint),
        )
    );

    let mut client = rpc::user::make_client(endpoint, tenant, requestor)
        .await
        .map_err(ErrorResponse::from)?;

    let index = id;
    client
        .show(Request::new(rpc::messages::EntityIndex { index }))
        .await
        .map(|msg| Json(msg.into_inner().into()))
        .map_err(ErrorResponse::from)
}

/// Route for creating a new user.
///
/// To use this route, use a POST request, sending as body a JSON containing the
/// expected data for creating a new user.
///
/// Upon success, returns the data for the created user as if it were requested
/// through the `show` method.
#[allow(unused_variables)]
#[openapi(tag = "User")]
#[post("/<tenant>/user", data = "<body>")]
async fn store(
    tenant: String,
    session: SessionInfo,
    body: Json<data::user::RecvUser>,
) -> RestResult<data::user::User> {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    let message: rpc::messages::User = body.0.into();

    if message.login == *"unknown" {
        return Err(ErrorResponse::BadRequest(
            crate::generic::Message::from("Username \"unknown\" is reserved").json(),
        ));
    }

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("POST /user: request USER.store ({})", endpoint),
        )
    );

    let mut client = rpc::user::make_client(endpoint, tenant, requestor)
        .await
        .map_err(ErrorResponse::from)?;

    client
        .store(Request::new(message))
        .await
        .map(|msg| Json(msg.into_inner().into()))
        .map_err(ErrorResponse::from)
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
#[allow(unused_variables)]
#[openapi(tag = "User")]
#[put("/<tenant>/user/<id>", data = "<body>")]
async fn update(
    tenant: String,
    session: SessionInfo,
    id: i32,
    body: Json<data::user::RecvUser>,
) -> RestResult<data::user::User> {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("PUT /user/<id>: request USER.update ({})", endpoint),
        )
    );

    let mut client = rpc::user::make_client(endpoint, tenant, requestor)
        .await
        .map_err(ErrorResponse::from)?;

    let mut message: rpc::messages::User = body.0.into();
    message.id = Some(id);

    client
        .update(Request::new(message))
        .await
        .map(|msg| Json(msg.into_inner().into()))
        .map_err(ErrorResponse::from)
}

/// Route for removing a user altogether.
///
/// To use this route, use a DELETE request. The ID of the user to be updated
/// should also be passed through the URL.
///
/// Upon success, returns a success message.
#[allow(unused_variables)]
#[openapi(tag = "User")]
#[delete("/<tenant>/user/<index>")]
async fn delete(
    tenant: String,
    session: SessionInfo,
    index: i32,
) -> RestResult<crate::generic::Message> {
    let endpoint = get_endpoint();
    let tenant = session.info.tenant.clone();
    let requestor = session.info.login.clone();

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("DELETE /user/<id>: request USER.delete ({})", endpoint),
        )
    );

    let mut client = rpc::user::make_client(endpoint, tenant, requestor)
        .await
        .map_err(ErrorResponse::from)?;

    client
        .delete(Request::new(rpc::messages::EntityIndex { index }))
        .await
        .map(|_| Json(crate::generic::Message::from("User removed successfully")))
        .map_err(ErrorResponse::from)
}
