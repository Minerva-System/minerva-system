use super::response::Response;
use rocket::Request;
use serde_json::json;

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![bad_request, unauthorized, not_found, internal]
}

fn gen_message(message: &str) -> String {
    json!({ "message": message }).to_string()
}

#[catch(400)]
fn bad_request(_req: &Request) -> Response {
    Response::BadRequest(gen_message("The provided request is not valid."))
}

#[catch(401)]
fn unauthorized(_req: &Request) -> Response {
    Response::Unauthorized(gen_message(
        "The request is missing authorization details. Check your credentials and try again.",
    ))
}

#[catch(404)]
fn not_found(_req: &Request) -> Response {
    Response::NotFound(gen_message(
        "The resource you are looking for does not exist.",
    ))
}

#[catch(500)]
fn internal(_req: &Request) -> Response {
    Response::InternalServerError(gen_message(
        "There was a problem in the service while processing your request.",
    ))
}
