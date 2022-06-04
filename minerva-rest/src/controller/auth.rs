//! This submodule describes routes for authentication and session creation
//! services.

use super::response;
use crate::utils;
use minerva_data as data;
use minerva_rpc as rpc;
use response::Response;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::Route;
use serde_json::json;
use std::env;
use tonic::Request;

/// Cookie name for the authentication token, namely the session identification.
pub static AUTH_COOKIE: &str = "auth_token";

/// Cookie name for the tenant name, saved within the browser.
pub static TENANT_COOKIE: &str = "tenant";

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
/// Upon a successful login attempt, the route will attempt to store session
/// cookies on the client.
///
/// # Request example
/// ```bash
/// curl -X POST 'http://localhost:9000/minerva/login' \
///      -H 'Content-Type: application/json' \
///      -d '{"login": "admin", "password": "admin"}' \
///      -c cookies.txt
/// ```
#[post("/<tenant>/login", data = "<body>")]
async fn login(
    tenant: &str,
    cookies: &CookieJar<'_>,
    body: Json<data::session::RecvSession>,
) -> Response {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    let body = body.as_new(tenant);
    let tenant = tenant.to_string();

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::LOGIN > SESSION::GENERATE @ {}", endpoint),
    );

    let mut client = rpc::session::make_client(endpoint, tenant.clone(), requestor).await;
    let response = client
        .generate(Request::new(body.clone().into()))
        .await
        .map(|msg| {
            let token = msg.into_inner().token;
            let auth_cookie = Cookie::new(AUTH_COOKIE, token.clone());
            let tenant_cookie = Cookie::new(TENANT_COOKIE, tenant.clone());
            cookies.add_private(auth_cookie);
            cookies.add(tenant_cookie);

            json!({ "token": token, "tenant": tenant })
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
/// curl -X POST http://localhost:9000/logoff \
///      -b cookies.txt
/// ```
#[post("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Response {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    let tenant = match utils::get_tenant(cookies) {
        Ok(tenant) => tenant,
        Err(response) => return response,
    };

    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::LOGOUT > SESSION::REMOVE @ {}", endpoint),
    );

    match cookies.get_private(AUTH_COOKIE) {
        Some(cookie) => {
            let mut client = rpc::session::make_client(endpoint, tenant, requestor).await;
            let token = cookie.value().to_string();
            let response = client
                .remove(Request::new(rpc::messages::SessionToken { token }))
                .await
                .map(|_| json!({ "message": "User logout successful" }));

            if response.is_ok() {
                cookies.remove_private(cookie);
                if let Some(cookie) = cookies.get(TENANT_COOKIE) {
                    cookies.remove(cookie.clone());
                }
            }

            Response::respond(response)
        }
        None => Response::Unauthorized(
            json!({
                "message": "Authentication token not found on session cookies",
            })
            .to_string(),
        ),
    }
}
