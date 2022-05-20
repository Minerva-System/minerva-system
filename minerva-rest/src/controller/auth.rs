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

static AUTH_COOKIE: &str = "token";

pub fn routes() -> Vec<Route> {
    routes![login, logout]
}

fn get_endpoint() -> String {
    let port = env::var("SESSION_SERVICE_PORT").expect("Unable to read SESSION_SERVICE_PORT");
    let srv = env::var("SESSION_SERVICE_SERVER").expect("Unable to read SESSION_SERVICE_SERVER");
    format!("http://{}:{}", srv, port)
}

#[post("/<tenant>/login", data = "<body>")]
pub async fn login(
    tenant: String,
    cookies: &CookieJar<'_>,
    body: Json<data::session::NewSession>,
) -> Response {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::LOGIN > SESSION::GENERATE @ {}", endpoint),
    );
    let mut client = rpc::session::make_client(endpoint, tenant, requestor).await;
    let response: Result<String, Status> = client
        .generate(Request::new((*body).clone().into()))
        .await
        .map(|msg| {
            let token = msg.into_inner().token;
            let cookie = Cookie::new(AUTH_COOKIE, token.clone());
            cookies.add_private(cookie);
            token
        });

    Response::respond(response)
}

#[post("/<tenant>/logout")]
pub async fn logout(tenant: String, cookies: &CookieJar<'_>) -> Response {
    let endpoint = get_endpoint();
    let requestor = "unknown".to_string();
    data::log::print(
        utils::get_ip(),
        requestor.clone(),
        tenant.clone(),
        &format!("REST::LOGIN > SESSION::GENERATE @ {}", endpoint),
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
            }

            Response::respond_empty(response)
        }
        None => Response::NotFound(
            json!({
                "message": "Authentication token couldn't be found in cookies"
            })
            .to_string(),
        ),
    }
}
