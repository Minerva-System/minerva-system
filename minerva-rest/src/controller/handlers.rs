//! This submodule contains default status code catchers for errors within the
//! API.

use super::response::Response;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use serde_json::json;

/// Exports an array of catcher functions to be used when mounting the server.
pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![
        default_catcher,
        bad_request,
        unauthorized,
        not_found,
        unprocessable_entity,
        internal,
        service_unavailable
    ]
}

/// Default error structure implementing a Rocket responder for any status code.
struct DefaultError {
    /// Status code for the error.
    status: Status,
}

impl rocket::response::Responder<'_, 'static> for DefaultError {
    fn respond_to(self, req: &Request<'_>) -> rocket::response::Result<'static> {
        let message = format!(
            "There was an error processing the request: {}",
            self.status.reason().unwrap()
        );
        rocket::response::Response::build_from(message.respond_to(req)?)
            .header(ContentType::JSON)
            .status(self.status)
            .ok()
    }
}

/// Generates a simple JSON string from a given message.
fn gen_message(message: &str) -> String {
    json!({ "message": message }).to_string()
}

/// Default error catcher for any kind of error that was not treated
/// specifically.
#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> DefaultError {
    DefaultError { status }
}

/// Error catcher for 400 - Bad Request.
#[catch(400)]
fn bad_request(status: Status, _req: &Request) -> Response {
    Response::BadRequest(gen_message(&format!(
        "The request is not valid: {}",
        status.reason().unwrap()
    )))
}

/// Error catcher for 401 - Unauthorized.
/// Returned on problems with the authentication fairing.
#[catch(401)]
fn unauthorized(status: Status, _req: &Request) -> Response {
    Response::Unauthorized(gen_message(&format!(
        "This request is unauthorized: {}",
        status.reason().unwrap(),
    )))
}

/// Error catcher for 404 - Not Found.
#[catch(404)]
fn not_found(status: Status, _req: &Request) -> Response {
    Response::NotFound(gen_message(&format!(
        "The resource you are looking for does not exist: {}",
        status.reason().unwrap(),
    )))
}

/// Error catcher for 412 - Unprocessable Entity.
/// Returned when a JSON body to a request cannot be deserialized.
#[catch(422)]
fn unprocessable_entity(status: Status, _req: &Request) -> Response {
    Response::UnprocessableEntity(gen_message(&format!(
        "The provided data could not be processed: {}",
        status.reason().unwrap(),
    )))
}

/// Error catcher for 500 - Internal Server Error.
#[catch(500)]
fn internal(status: Status, _req: &Request) -> Response {
    Response::InternalServerError(gen_message(&format!(
        "There was a problem in the service while processing your request: {}",
        status.reason().unwrap(),
    )))
}

/// Error catcher for 503 - Service Unavailable.
/// Returned whenever a microservice cannot be reached to complete the request.
#[catch(503)]
fn service_unavailable(status: Status, _req: &Request) -> Response {
    Response::ServiceUnavailable(gen_message(&format!(
        "The service is currently unavailable: {}",
        status.reason().unwrap(),
    )))
}
