//! This submodule contains structures and helper functions for creating
//! responses for a request.

use crate::generic::Message;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{MediaType, Responses},
    response::OpenApiResponderInner,
};
use tonic::{Code, Status};

/// Enumeration containing many kinds of error responses to a REST request that
/// was received. All of these responses should contain an embedded JSON in
/// String format. These error responses are modelled after HTTP response codes.
#[derive(Responder, Debug, Clone)]
pub enum ErrorResponse {
    /// 400 Bad Request
    #[response(status = 400, content_type = "json")]
    BadRequest(Message),

    /// 401 Unauthorized
    #[response(status = 401, content_type = "json")]
    Unauthorized(Message),

    /// 404 Not Found
    #[response(status = 404, content_type = "json")]
    NotFound(Message),

    /// 408 Request Timeout
    #[response(status = 408, content_type = "json")]
    RequestTimeout(Message),

    /// 409 Conflict
    #[response(status = 409, content_type = "json")]
    Conflict(Message),

    /// 412 Precondition Failed
    #[response(status = 412, content_type = "json")]
    PreconditionFailed(Message),

    /// 422 Unprocessable Entity
    #[response(status = 422, content_type = "json")]
    UnprocessableEntity(Message),

    /// 444 No Response
    #[response(status = 444, content_type = "json")]
    NoResponse(Message),

    /// 499 Client Closed Request
    #[response(status = 499, content_type = "json")]
    ClientClosedRequest(Message),

    /// 500 Internal Server Error
    #[response(status = 500, content_type = "json")]
    InternalServerError(Message),

    /// 501 Not Implemented
    #[response(status = 501, content_type = "json")]
    NotImplemented(Message),

    /// 503 Service Unavailable
    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(Message),

    /// 511 Network Authentication Required
    #[response(status = 511, content_type = "json")]
    NetworkAuthenticationRequired(Message),
}

/// Generic Result type for responses on REST routes.
pub type RestResult<T> = Result<Json<T>, ErrorResponse>;

impl ErrorResponse {
    /// Actual internal conversion function for generating an error `Response`
    /// from a gRPC `Status`. The `Status` message will be converted into a JSON
    /// object containing a single `"message"` field, which will be the response
    /// body.
    ///
    /// # Panics
    /// This function will panic if using an unhandled status code or if the
    /// status code is "`Ok`", in which case it should have been a successful
    /// response instead.
    fn convert(status: Status) -> Self {
        let message = Message::from(status.message());

        match status.code() {
            Code::Aborted => Self::NoResponse(message),
            Code::AlreadyExists => Self::Conflict(message),
            Code::Cancelled => Self::ClientClosedRequest(message),
            Code::DataLoss => Self::BadRequest(message),
            Code::DeadlineExceeded => Self::RequestTimeout(message),
            Code::FailedPrecondition => Self::PreconditionFailed(message),
            Code::Internal => Self::InternalServerError(message),
            Code::InvalidArgument => Self::UnprocessableEntity(message),
            Code::NotFound => Self::NotFound(message),
            Code::Ok => panic!("Returned an error with an 'OK' status. What???"),
            Code::OutOfRange => Self::UnprocessableEntity(message),
            Code::PermissionDenied => Self::Unauthorized(message),
            Code::Unauthenticated => Self::NetworkAuthenticationRequired(message),
            Code::Unavailable => Self::ServiceUnavailable(message),
            Code::Unimplemented => Self::NotImplemented(message),
            Code::Unknown => Self::InternalServerError(message),
            _ => panic!("Unhandled return status: {}", status),
        }
    }
}

impl From<Status> for ErrorResponse {
    fn from(status: Status) -> Self {
        Self::convert(status)
    }
}

/// Generates an OpenAPI 3 response object, given a title and a body.
/// This function also expects that the response content is always in JSON format.
fn gen_response(
    gen: &mut OpenApiGenerator,
    title: &str,
    body: &str,
) -> rocket_okapi::okapi::openapi3::Response {
    use rocket_okapi::okapi::{self, map};
    let schema = gen.json_schema::<crate::generic::Message>();
    rocket_okapi::okapi::openapi3::Response {
        description: format!("### {}\n{}", title, body),
        content: map! {
            "application/json".to_owned() => MediaType {
            schema: Some(schema),
            ..Default::default()
            }
        },
        ..Default::default()
    }
}

impl OpenApiResponderInner for ErrorResponse {
    fn responses(gen: &mut rocket_okapi::gen::OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        use rocket_okapi::okapi::openapi3::RefOr;
        use rocket_okapi::okapi::{self, map};

        let r400 = gen_response(
            gen,
            "Bad Request",
            "Request was not performed with expected conventions",
        );

        let r401 = gen_response(
            gen,
            "Unauthorized",
            "The request was missing authentication on headers",
        );

        let r404 = gen_response(gen, "Not Found", "The desired resource was not found");

        let r408 = gen_response(
            gen,
            "Request Timeout",
            "The connection timed out while requesting data from the service",
        );

        let r409 = gen_response(
            gen,
            "Conflict",
            "There was a conflict while managing the entity, possibly related to duplication",
        );

        let r412 = gen_response(gen, "Precondition Failed", "There were inconsistencies while processing the entity, probably related to some business rule");

        let r422 = gen_response(gen, "Unprocessable Entity", "The payload could not be processed, or possibly did not contain data on the expected format");

        let r444 = gen_response(
            gen,
            "No Response",
            "The connection was aborted by the service",
        );

        let r499 = gen_response(gen, "Client Closed Request", "The request was cancelled");

        let r500 = gen_response(
            gen,
            "Internal Server Error",
            "An unknown error has occurred on the system",
        );

        let r501 = gen_response(
            gen,
            "Not Implemented",
            "This feature is still not implemented",
        );

        let r503 = gen_response(
            gen,
            "Service Unavailable",
            "The requested service is unavailable for connection",
        );

        let r511 = gen_response(
            gen,
            "Network Authentication Required",
            "The requested service needs internal authentication, which was not provided",
        );

        #[rustfmt::skip]
        let responses = Responses {
            responses: map! {
                "400".to_owned() => RefOr::Object(r400),
                "401".to_owned() => RefOr::Object(r401),
		"404".to_owned() => RefOr::Object(r404),
		"408".to_owned() => RefOr::Object(r408),
		"409".to_owned() => RefOr::Object(r409),
		"412".to_owned() => RefOr::Object(r412),
		"422".to_owned() => RefOr::Object(r422),
                "444".to_owned() => RefOr::Object(r444),
                "499".to_owned() => RefOr::Object(r499),
                "500".to_owned() => RefOr::Object(r500),
                "501".to_owned() => RefOr::Object(r501),
                "503".to_owned() => RefOr::Object(r503),
                "511".to_owned() => RefOr::Object(r511),
            },
            ..Default::default()
        };

        Ok(responses)
    }
}
