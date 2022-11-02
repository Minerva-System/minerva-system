//! This submodule contains the implementation of an authentication fairing.

use minerva_data::session::Session;
use minerva_rpc as rpc;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

/// Struct for session information that can be retrieved for every access to a
/// route that explicitly retrieves it.
///
/// Adding a `State` containing a `SessionInfo` to a route function's parameters
/// automatically means that the route will only be accessible by an
/// authenticated user that passes its authentication info through cookies. If
/// a `SessionInfo` can be retrieved given the cookies information, then the
/// user is authenticated for using that route.
pub struct SessionInfo {
    /// Session info for the authenticated user.
    pub info: Session,
    /// Session token for the authenticated user.
    pub token: String,
}

impl SessionInfo {
    /// Generates a new session info from a `SessionData` gRPC message.
    pub fn from(info: rpc::messages::SessionData, token: String) -> Self {
        Self {
            info: info.into(),
            token,
        }
    }
}

/// Describes kinds of errors when attempting to retrieve a `SessionInfo`
/// through the workings of its fairing.
#[derive(Debug, Clone)]
pub enum SessionError {
    /// The cookies did not contain information on the tenant.
    MissingTenant,
    /// There is no authentication data for that user's session.
    MissingAuth,
    /// The authentication data has expired.
    ExpiredAuth,
    /// The required microservice could not be reached.
    ServiceUnreachable,
}

/// Extracts bearer token from authorization header, if any.
fn get_bearer_token(authorization: &str) -> Option<String> {
    let prefix = "Bearer ";
    if authorization.starts_with(prefix) {
        Some(authorization.trim_start_matches(prefix).to_owned())
    } else {
        None
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionInfo {
    type Error = SessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get tenant from first segment of uri, e.g. /<tenant>/...
        let tenant = match req.uri().path().segments().get(0) {
            Some(tenant) => tenant.to_owned(),
            None => return Outcome::Failure((Status::Unauthorized, SessionError::MissingTenant)),
        };

        // Get auth token from request header
        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::BadRequest, SessionError::MissingAuth)),
            Some(header) => match get_bearer_token(header) {
                Some(token) => {
                    // Request a connection to SESSION service
                    let endpoint = crate::controller::auth::get_endpoint();
                    let msg = rpc::messages::SessionToken {
                        token: token.clone(),
                    };
                    let requestor = "unknown".into();

                    match rpc::session::make_client(endpoint, tenant, requestor).await {
                        Err(_) => {
                            return Outcome::Failure((
                                Status::ServiceUnavailable,
                                SessionError::ServiceUnreachable,
                            ))
                        }
                        // Upon success, attempt to fetch session data
                        Ok(mut client) => match client.retrieve(tonic::Request::new(msg)).await {
                            Ok(response) => {
                                Outcome::Success(SessionInfo::from(response.into_inner(), token))
                            }
                            Err(_) => {
                                Outcome::Failure((Status::Unauthorized, SessionError::ExpiredAuth))
                            }
                        },
                    }
                }
                None => Outcome::Failure((Status::Unauthorized, SessionError::MissingAuth)),
            },
        }
    }
}
