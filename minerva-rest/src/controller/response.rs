//! This submodule contains structures and helper functions for creating
//! responses for a request.

use rocket::response::Responder;
use serde::Serialize;
use serde_json::json;
use tonic::{Code, Status};

/// Enumeration containing many kinds of responses to a REST request that was
/// received. All of these responses should contain an embedded JSON in String
/// format. These responses are modelled after HTTP response codes.
#[allow(dead_code)]
#[derive(Responder, Debug, Clone)]
pub enum Response {
    /// 200 OK
    #[response(status = 200, content_type = "json")]
    Ok(String),

    /// 304 Not Modified
    #[response(status = 304, content_type = "json")]
    NotModified(String),

    /// 400 Bad Request
    #[response(status = 400, content_type = "json")]
    BadRequest(String),

    /// 401 Unauthorized
    #[response(status = 401, content_type = "json")]
    Unauthorized(String),

    /// 404 Not Found
    #[response(status = 404, content_type = "json")]
    NotFound(String),

    /// 408 Request Timeout
    #[response(status = 408, content_type = "json")]
    RequestTimeout(String),

    /// 409 Conflict
    #[response(status = 409, content_type = "json")]
    Conflict(String),

    /// 412 Precondition Failed
    #[response(status = 412, content_type = "json")]
    PreconditionFailed(String),

    /// 444 No Response
    #[response(status = 444, content_type = "json")]
    NoResponse(String),

    /// 499 Client Closed Request
    #[response(status = 499, content_type = "json")]
    ClientClosedRequest(String),

    /// 500 Internal Server Error
    #[response(status = 500, content_type = "json")]
    InternalServerError(String),

    /// 501 Not Implemented
    #[response(status = 501, content_type = "json")]
    NotImplemented(String),

    /// 503 Service Unavailable
    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(String),

    /// 511 Network Authentication Required
    #[response(status = 511, content_type = "json")]
    NetworkAuthenticationRequired(String),
}

impl Response {
    /// Generates a `Response` from a gRPC response. This assumes that the
    /// gRPC response is a `Result` that either holds a serializable success
    /// value that will be serialized to JSON, or an error `Status` in case of
    /// failure which will be converted to an appropriate HTTP error, containing
    /// the `Status`'s message.
    pub fn respond<T: Serialize>(response: Result<T, Status>) -> Self {
        match response {
            Ok(object) => Response::Ok(serde_json::to_string(&object).unwrap()),
            Err(status) => Self::convert(status),
        }
    }

    /// Generates a `Response` from a gRPC response, whenever the response
    /// success result is empty. In that case, an empty JSON object is returned.
    /// If the gRPC response is an error `Status`, that error will be converted
    /// to an appropriated HTTP error, containing the `Status`'s message.
    pub fn respond_empty(response: Result<tonic::Response<()>, Status>) -> Self {
        match response {
            Ok(_) => Response::Ok("{}".into()),
            Err(status) => Self::convert(status),
        }
    }

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
        let message = json!({ "message": status.message() }).to_string();
        match status.code() {
            Code::Aborted => Self::NoResponse(message),
            Code::AlreadyExists => Self::Conflict(message),
            Code::Cancelled => Self::ClientClosedRequest(message),
            Code::DataLoss => Self::BadRequest(message),
            Code::DeadlineExceeded => Self::RequestTimeout(message),
            Code::FailedPrecondition => Self::PreconditionFailed(message),
            Code::Internal => Self::InternalServerError(message),
            Code::InvalidArgument => Self::BadRequest(message),
            Code::NotFound => Self::NotFound(message),
            Code::Ok => panic!("Returned an error with an 'OK' status. What???"),
            Code::OutOfRange => Self::BadRequest(message),
            Code::PermissionDenied => Self::Unauthorized(message),
            Code::Unauthenticated => Self::NetworkAuthenticationRequired(message),
            Code::Unavailable => Self::ServiceUnavailable(message),
            Code::Unimplemented => Self::NotImplemented(message),
            Code::Unknown => Self::InternalServerError(message),
            _ => panic!("Unhandled return status: {}", status),
        }
    }
}
