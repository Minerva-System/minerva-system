use super::response::Response;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::Request;
use serde_json::json;

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

struct DefaultError {
    status: Status,
}

impl<'r> rocket::response::Responder<'r, 'static> for DefaultError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
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

fn gen_message(message: &str) -> String {
    json!({ "message": message }).to_string()
}

#[catch(default)]
fn default_catcher<'r>(status: Status, _req: &Request) -> DefaultError {
    DefaultError { status }
}

#[catch(400)]
fn bad_request(status: Status, _req: &Request) -> Response {
    Response::BadRequest(gen_message(&format!(
        "The request is not valid: {}",
        status.reason().unwrap()
    )))
}

#[catch(401)]
fn unauthorized(status: Status, _req: &Request) -> Response {
    Response::Unauthorized(gen_message(&format!(
        "This request is unauthorized: {}",
        status.reason().unwrap(),
    )))
}

#[catch(404)]
fn not_found(status: Status, _req: &Request) -> Response {
    Response::NotFound(gen_message(&format!(
        "The resource you are looking for does not exist: {}",
        status.reason().unwrap(),
    )))
}

#[catch(422)]
fn unprocessable_entity(status: Status, _req: &Request) -> Response {
    Response::UnprocessableEntity(gen_message(&format!(
        "The provided data could not be processed: {}",
        status.reason().unwrap(),
    )))
}

#[catch(500)]
fn internal(status: Status, _req: &Request) -> Response {
    Response::InternalServerError(gen_message(&format!(
        "There was a problem in the service while processing your request: {}",
        status.reason().unwrap(),
    )))
}

#[catch(503)]
fn service_unavailable(status: Status, _req: &Request) -> Response {
    Response::ServiceUnavailable(gen_message(&format!(
        "The service is currently unavailable: {}",
        status.reason().unwrap(),
    )))
}
