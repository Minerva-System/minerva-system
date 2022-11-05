//! This module contains the actual implementation for the `User` gRPC service.

use crate::repository;
use minerva_broker as broker;
use minerva_data as lib_data;
use minerva_data::db::DBPool;
use minerva_rpc as lib_rpc;
use minerva_rpc::user::user_server::User;
use minerva_rpc::{messages, metadata};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

/// Represents a gRPC service for user.
#[derive(Clone)]
pub struct UserService {
    /// Holds database connection pools for all tenants.
    pub pools: HashMap<String, (DBPool, broker::LapinPool)>,
}

#[tonic::async_trait]
impl User for UserService {
    async fn index(
        &self,
        req: Request<messages::PageIndex>,
    ) -> Result<Response<messages::UserList>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "USER::INDEX",
        );

        let page = req.into_inner().index.unwrap_or(0);

        let result = {
            let (dbpool, _rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::get_list(page, &connection)
                .map_err(|e| Status::internal(format!("Cannot recover user list: {}", e)))?
        };

        Ok(Response::new(minerva_data::user::vec_to_message(result)))
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::User>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "USER::SHOW",
        );

        let result = {
            let (dbpool, _rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::get_user(req.get_ref().index, &connection)
                .map_err(|e| Status::internal(format!("Cannot recover user: {}", e)))?
        };

        if let Some(user) = result {
            Ok(Response::new(user.into()))
        } else {
            Err(Status::not_found("User not found."))
        }
    }

    async fn store(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "USER::STORE",
        );

        let result = {
            let data = req.into_inner().into();

            let (dbpool, _rmqserver) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::add_user(data, requestor, &connection)
        };

        result
            .map(|u| Response::new(u.into()))
            .map_err(|e| Status::failed_precondition(format!("Unable to register user: {}", e)))
    }

    async fn update(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "USER::UPDATE",
        );

        let result = {
            let data = req.into_inner().into();

            let (dbpool, _rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::update_user(data, requestor, &connection)
        };

        result
            .map(|u| Response::new(u.into()))
            .map_err(|e| Status::failed_precondition(format!("Unable to register user: {}", e)))
    }

    async fn delete(&self, req: Request<messages::EntityIndex>) -> Result<Response<()>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant")
            .ok_or_else(|| Status::failed_precondition("Missing tenant on request metadata"))?;

        let requestor = metadata::get_value(req.metadata(), "requestor")
            .ok_or_else(|| Status::failed_precondition("Missing requestor on request metadata"))?;

        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "USER::DELETE",
        );

        let result = {
            let (dbpool, rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let rabbitmq = rmqpool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Could not connect to RabbitMQ: {}", e)))?;

            repository::delete_user(req.get_ref().index, requestor, dbpool, &rabbitmq).await
        };

        result.map(|_| Response::new(()))
    }
}
