use minerva_data::session::Session;
use minerva_rpc as rpc;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct SessionInfo {
    pub info: Session,
}

impl SessionInfo {
    pub fn from(info: rpc::messages::SessionData) -> Self {
        Self { info: info.into() }
    }
}

#[derive(Debug, Clone)]
pub enum SessionError {
    MissingTenant,
    MissingAuth,
    ExpiredAuth,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionInfo {
    type Error = SessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let tenant = match req.cookies().get(crate::controller::auth::TENANT_COOKIE) {
            Some(cookie) => cookie.value().to_string(),
            None => return Outcome::Failure((Status::BadRequest, SessionError::MissingTenant)),
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
                let mut client = rpc::session::make_client(endpoint, tenant, requestor).await;

                match client.retrieve(tonic::Request::new(token)).await {
                    Ok(response) => Outcome::Success(SessionInfo::from(response.into_inner())),
                    Err(_) => Outcome::Failure((Status::Unauthorized, SessionError::ExpiredAuth)),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, SessionError::MissingAuth)),
        }
    }
}
