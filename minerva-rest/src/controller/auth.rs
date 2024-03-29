//! This submodule describes routes for authentication and session creation
//! services.

use super::response;
use crate::fairings::auth::SessionInfo;
use crate::utils;
use data::session::SessionResponse;
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
    openapi_get_routes_spec![login, logout]
}

/// Retrieves the endpoint for the gRPC session service. Requires that the proper
/// environment variables are defined.
pub fn get_endpoint() -> String {
    let port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let srv = env::var("SESSION_SERVICE_SERVER").expect("Unable to read SESSION_SERVICE_SERVER");
    format!("http://{}:{}", srv, port)
}

/// Route for user login.
///
/// This route requires that the tenant is informed on the login route.
/// Furthermore, login needs login data for the creation of a session.
///
/// Upon a successful login attempt, the route will return the tenant and the
/// session token data for the current user.
#[openapi(tag = "Authentication")]
#[post("/<tenant>/login", data = "<body>")]
async fn login(
    tenant: &str,
    body: Json<data::session::RecvSession>,
) -> RestResult<SessionResponse> {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    let body = body.as_new(tenant);
    let tenant = tenant.to_string();

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("POST /login: request SESSION.generate ({})", endpoint),
        )
    );

    let mut client = rpc::session::make_client(endpoint, tenant.clone(), requestor)
        .await
        .map_err(|status| {
            error!("Error while connecting to SESSION: {:?}", status);
            ErrorResponse::from(status)
        })?;

    client
        .generate(Request::new(body.clone().into()))
        .await
        .map(|msg| {
            let token = msg.into_inner().token;
            Json(SessionResponse { token, tenant })
        })
        .map_err(|status| {
            error!("Error while creating session: {:?}", status);
            ErrorResponse::from(status)
        })
}

/// Route for user logoff.
///
/// This route requires a session token passed as Bearer Token. Upon successful
/// logoff, the session will be invalidated on both database and cache.
#[allow(unused_variables)]
#[openapi(tag = "Authentication")]
#[post("/<tenant>/logout")]
async fn logout(tenant: String, session: SessionInfo) -> RestResult<crate::generic::Message> {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    let tenant = session.info.tenant;

    info!(
        "{}",
        data::log::format(
            utils::get_ip(),
            &requestor,
            &tenant,
            &format!("POST /logout: request SESSION.remove ({})", endpoint),
        )
    );

    let mut client = rpc::session::make_client(endpoint, tenant.clone(), requestor)
        .await
        .map_err(ErrorResponse::from)?;

    let token = session.token.clone();

    client
        .remove(Request::new(rpc::messages::SessionToken { token }))
        .await
        .map(|_| Json(crate::generic::Message::from("User logout successful")))
        .map_err(ErrorResponse::from)
}
