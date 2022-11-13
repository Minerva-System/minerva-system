//! This submodule describes routes for authentication and session creation
//! services.

use super::response;
use crate::fairings::auth::SessionInfo;
use crate::utils;
use log::info;
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
    routes![login, logout]
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
///
/// # Request example
/// ```bash
/// curl -X POST 'http://localhost:9000/minerva/login' \
///      -H 'Content-Type: application/json' \
///      -d '{"login": "admin", "password": "admin"}'
/// ```
#[post("/<tenant>/login", data = "<body>")]
async fn login(tenant: &str, body: Json<data::session::RecvSession>) -> Response {
    use data::session::SessionResponse;
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

    let client = rpc::session::make_client(endpoint, tenant.clone(), requestor).await;
    if client.is_err() {
        return Response::generate_error(client);
    }
    let mut client = client.unwrap();

    let response = client
        .generate(Request::new(body.clone().into()))
        .await
        .map(|msg| {
            let token = msg.into_inner().token;

            SessionResponse { token, tenant }
        });

    Response::respond(response)
}

/// Route for user logoff.
///
/// This route requires that session cookies exist on the client requesting
/// logoff. These cookies will be then accessed by the server and, upon
/// successful logoff, will be deleted from the client's cookie jar.
///
/// # Request example
/// ```bash
/// curl -X POST http://localhost:9000/minerva/logout \
///      -H 'Authorization: Bearer {token}'
/// ```
#[post("/<_>/logout")]
async fn logout(session: SessionInfo) -> Response {
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

    match rpc::session::make_client(endpoint, tenant.clone(), requestor).await {
        Ok(mut client) => {
            let token = session.token.clone();
            let response = client
                .remove(Request::new(rpc::messages::SessionToken { token }))
                .await
                .map(|_| json!({ "message": "User logout successful" }));

            Response::respond(response)
        }
        err => Response::generate_error(err),
    }
}
