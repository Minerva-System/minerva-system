//! This module contains the actual implementation for the `Session`
//! gRPC service.

use crate::repository;
use minerva_data as lib_data;
use minerva_data::db::DBPool;
use minerva_rpc as lib_rpc;
use minerva_rpc::session::session_server::Session;
use minerva_rpc::{messages, metadata};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

/// Represents a gRPC service for session.
#[derive(Clone)]
pub struct SessionService {
    /// Holds database (relational and non-relational) connection pools for
    /// all tenants.
    pub pools: HashMap<String, (DBPool, mongodb::Client, redis::Client)>,
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
            let (postgres, mongodb, redis) =
                self.pools.get(&tenant).expect("Unable to find tenant");
            let mongo = mongodb.database(&tenant);
            repository::create_session(&tenant, data, postgres.clone(), mongo, redis).await
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

        let token = req.into_inner().token;
        let (_, mongodb, redis) = self.pools.get(&tenant).expect("Unable to find tenant");
        let mongo = mongodb.database(&tenant);
        let session = repository::recover_session(&tenant, token, mongo, redis).await?;
        Ok(Response::new(session.into()))
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

        let token = req.into_inner().token;
        let (postgres, mongodb, redis) = self.pools.get(&tenant).expect("Unable to find tenant");
        let mongo = mongodb.database(&tenant);
        let _ = repository::remove_session(&tenant, token, postgres.clone(), mongo, redis).await?;
        Ok(Response::new(()))
    }
}
