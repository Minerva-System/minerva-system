use crate::controller::response::Response;
use rocket::http::CookieJar;
use serde_json::json;
use std::net::SocketAddr;

pub fn get_ip() -> SocketAddr {
    use std::net::{IpAddr, Ipv4Addr};
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000)
}

#[allow(dead_code)]
pub fn get_tenant(cookies: &CookieJar<'_>) -> Result<String, Response> {
    match cookies.get(crate::controller::auth::TENANT_COOKIE) {
        Some(cookie) => Ok(cookie.value().to_string()),
        None => Err(Response::BadRequest(
            json!({
                "message": "Tenant name not found on session cookies",
            })
            .to_string(),
        )),
    }
}
