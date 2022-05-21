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
use tonic::{Request, Status};

pub static AUTH_COOKIE: &str = "auth_token";
pub static TENANT_COOKIE: &str = "tenant";

pub fn routes() -> Vec<Route> {
    routes![login, logout]
}

pub fn get_endpoint() -> String {
    let port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let srv = env::var("SESSION_SERVICE_SERVER").expect("Unable to read SESSION_SERVICE_SERVER");
    format!("http://{}:{}", srv, port)
}

#[post("/<tenant>/login", data = "<body>")]
pub async fn login(
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
    let response: Result<String, Status> = client
        .generate(Request::new(body.clone().into()))
        .await
        .map(|msg| {
            let token = msg.into_inner().token;
            let auth_cookie = Cookie::new(AUTH_COOKIE, token.clone());
            let tenant_cookie = Cookie::new(TENANT_COOKIE, tenant.clone());
            cookies.add_private(auth_cookie);
            cookies.add(tenant_cookie);
            token
        });

    Response::respond(response)
}

#[post("/<tenant>/logout")]
pub async fn logout(tenant: &str, cookies: &CookieJar<'_>) -> Response {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    let tenant = tenant.to_string();

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
                .await;

            if response.is_ok() {
                cookies.remove_private(cookie);
                if let Some(cookie) = cookies.get(TENANT_COOKIE) {
                    cookies.remove(cookie.clone());
                }
            }

            Response::respond_empty(response)
        }
        None => Response::Unauthorized(
            json!({
                "message": "Authentication token not found on session cookies",
            })
            .to_string(),
        ),
    }
}
