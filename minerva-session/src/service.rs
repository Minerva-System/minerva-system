use crate::repository;
use minerva_data as lib_data;
use minerva_data::db::DBPool;
use minerva_rpc as lib_rpc;
use minerva_rpc::session::session_server::Session;
use minerva_rpc::{messages, metadata};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct SessionService {
    pub pools: HashMap<String, (DBPool, mongodb::Client)>,
}

#[tonic::async_trait]
impl Session for SessionService {
    async fn generate(
        &self,
        req: Request<messages::SessionCreationData>,
    ) -> Result<Response<messages::SessionToken>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "SESSION::GENERATE",
        );

        let data = req.into_inner().into();

        let token = {
            let (postgres, mongodb) = self.pools.get(&tenant).expect("Unable to find tenant");
            let mongo = mongodb.database(&tenant);
            repository::create_session(data, postgres.clone(), mongo).await
        }?;

        Ok(Response::new(messages::SessionToken { token }))
    }

    async fn retrieve(
        &self,
        req: Request<messages::SessionToken>,
    ) -> Result<Response<messages::SessionData>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "SESSION::RETRIEVE",
        );

        todo!();
    }

    async fn remove(&self, req: Request<messages::SessionToken>) -> Result<Response<()>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "SESSION::REMOVE",
        );

        todo!();
    }
}
