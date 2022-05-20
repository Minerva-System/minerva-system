use rocket::response::Responder;
use serde::Serialize;
use serde_json::json;
use tonic::{Code, Status};

#[allow(dead_code)]
#[derive(Responder, Debug, Clone)]
pub enum Response {
    #[response(status = 200, content_type = "json")]
    Ok(String),
    #[response(status = 400, content_type = "json")]
    BadRequest(String),
    #[response(status = 401, content_type = "json")]
    Unauthorized(String),
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[response(status = 500, content_type = "json")]
    InternalServerError(String),
    #[response(status = 501, content_type = "json")]
    NotImplemented(String),
    #[response(status = 408, content_type = "json")]
    RequestTimeout(String),
    #[response(status = 511, content_type = "json")]
    NetworkAuthenticationRequired(String),
    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(String),
    #[response(status = 412, content_type = "json")]
    PreconditionFailed(String),
    #[response(status = 304, content_type = "json")]
    NotModified(String),
    #[response(status = 499, content_type = "json")]
    ClientClosedRequest(String),
    #[response(status = 409, content_type = "json")]
    Conflict(String),
    #[response(status = 444, content_type = "json")]
    NoResponse(String),
}

impl Response {
    pub fn respond<T: Serialize>(response: Result<T, Status>) -> Self {
        match response {
            Ok(object) => Response::Ok(serde_json::to_string(&object).unwrap()),
            Err(status) => Self::convert(status),
        }
    }

    pub fn respond_empty(response: Result<tonic::Response<()>, Status>) -> Self {
        match response {
            Ok(_) => Response::Ok("{}".into()),
            Err(status) => Self::convert(status),
        }
    }

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
