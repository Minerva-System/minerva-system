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
}

impl SessionInfo {
    /// Generates a new session info from a `SessionData` gRPC message.
    pub fn from(info: rpc::messages::SessionData) -> Self {
        Self { info: info.into() }
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionInfo {
    type Error = SessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let tenant = match req.cookies().get(crate::controller::auth::TENANT_COOKIE) {
            Some(cookie) => cookie.value().to_string(),
            None => return Outcome::Failure((Status::Unauthorized, SessionError::MissingTenant)),
        };

        match req
            .cookies()
            .get_private(crate::controller::auth::AUTH_COOKIE)
        {
            Some(cookie) => {
                let endpoint = crate::controller::auth::get_endpoint();
                let token = rpc::messages::SessionToken {
                    token: cookie.value().to_string(),
                };
                let requestor = "unknown".into();
                let client = rpc::session::make_client(endpoint, tenant, requestor).await;
                if client.is_err() {
                    return Outcome::Failure((
                        Status::ServiceUnavailable,
                        SessionError::ServiceUnreachable,
                    ));
                }
                let mut client = client.unwrap();

                match client.retrieve(tonic::Request::new(token)).await {
                    Ok(response) => Outcome::Success(SessionInfo::from(response.into_inner())),
                    Err(_) => Outcome::Failure((Status::Unauthorized, SessionError::ExpiredAuth)),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, SessionError::MissingAuth)),
        }
    }
}
