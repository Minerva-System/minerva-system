//! This submodule contains structures and helper functions for creating
//! responses for a request.

use crate::generic::Message;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::Responses, response::OpenApiResponderInner};
use tonic::{Code, Status};

/// Enumeration containing many kinds of error responses to a REST request that
/// was received. All of these responses should contain an embedded JSON in
/// String format. These error responses are modelled after HTTP response codes.
#[allow(dead_code)]
#[derive(Responder, Debug, Clone)]
pub enum ErrorResponse {
    /// 304 Not Modified
    #[response(status = 304, content_type = "json")]
    NotModified(Message),

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

// TODO: The correct way to handle this is to actually have the API return something like
// Result<Json<WhateverResponsePayload>, Json<ErrorResponse>>.
// This would need refactoring of Response into ErrorResponse.
impl OpenApiResponderInner for ErrorResponse {
    fn responses(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<Responses> {
        Ok(Responses::default())
    }
}
