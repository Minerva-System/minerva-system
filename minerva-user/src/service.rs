//! This module contains the actual implementation for the `User` gRPC service.

use crate::repository;
use log::{error, info};
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
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or_else(|| {
            error!("Tenant not found on request metadata!");
            Status::failed_precondition("Missing tenant on request metadata")
        })?;

        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or_else(|| {
            error!("Requestor not found on request metadata!");
            Status::failed_precondition("Missing requestor on request metadata")
        })?;

        info!(
            "{}",
            lib_data::log::format(
                lib_rpc::get_address(&req),
                &requestor,
                &tenant,
                "get user index"
            )
        );

        let page = req.into_inner().index.unwrap_or(0);

        let result = {
            let (dbpool, _rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool.get().await.map_err(|e| {
                error!("Database access error: {}", e);
                Status::internal("There was an error while accessing the database")
            })?;

            repository::get_list(page, &connection).map_err(|e| {
                error!("Could not recover user list: {}", e);
                Status::internal("There was an error while recovering the user list")
            })?
        };

        Ok(Response::new(minerva_data::user::vec_to_message(result)))
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::User>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or_else(|| {
            error!("Tenant not found on request metadata!");
            Status::failed_precondition("Missing tenant on request metadata")
        })?;

        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or_else(|| {
            error!("Requestor not found on request metadata!");
            Status::failed_precondition("Missing requestor on request metadata")
        })?;

        info!(
            "{}",
            lib_data::log::format(lib_rpc::get_address(&req), &requestor, &tenant, "show user")
        );

        let result = {
            let (dbpool, _rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool.get().await.map_err(|e| {
                error!("Database access error: {}", e);
                Status::internal("There was an error while accessing the database")
            })?;

            repository::get_user(req.get_ref().index, &connection).map_err(|e| {
                error!("Cannot recover user: {}", e);
                Status::internal("There was an error while trying to recover user data")
            })?
        };

        if let Some(user) = result {
            Ok(Response::new(user.into()))
        } else {
            Err(Status::not_found("User not found"))
        }
    }

    async fn store(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or_else(|| {
            error!("Tenant not found on request metadata!");
            Status::failed_precondition("Missing tenant on request metadata")
        })?;

        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or_else(|| {
            error!("Requestor not found on request metadata!");
            Status::failed_precondition("Missing requestor on request metadata")
        })?;

        info!(
            "{}",
            lib_data::log::format(
                lib_rpc::get_address(&req),
                &requestor,
                &tenant,
                "store user"
            )
        );

        let result = {
            let data = req.into_inner().into();

            let (dbpool, _rmqserver) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool.get().await.map_err(|e| {
                error!("Database access error: {}", e);
                Status::internal("There was an error while accessing the database")
            })?;

            repository::add_user(data, requestor, &connection)
        };

        result.map(|u| Response::new(u.into())).map_err(|e| {
            error!("Unable to register user (possibly already exists): {}", e);

            // Assume that the user already exists, at this point.
            // Possibly a conflict on unique keys.
            Status::already_exists("This username or e-mail already exists")
        })
    }

    async fn update(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or_else(|| {
            error!("Tenant not found on request metadata!");
            Status::failed_precondition("Missing tenant on request metadata")
        })?;

        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or_else(|| {
            error!("Requestor not found on request metadata!");
            Status::failed_precondition("Missing requestor on request metadata")
        })?;

        info!(
            "{}",
            lib_data::log::format(
                lib_rpc::get_address(&req),
                &requestor,
                &tenant,
                "update user"
            )
        );

        let result = {
            let data = req.into_inner().into();

            let (dbpool, _rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let connection = dbpool.get().await.map_err(|e| {
                error!("Database access error: {}", e);
                Status::internal("There was an error while accessing the database")
            })?;

            repository::update_user(data, requestor, &connection)
        };

        result.map(|u| Response::new(u.into())).map_err(|e| {
            error!("Unable to register user: {}", e);
            Status::failed_precondition("There was an error while trying to create the new user")
        })
    }

    async fn delete(&self, req: Request<messages::EntityIndex>) -> Result<Response<()>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or_else(|| {
            error!("Tenant not found on request metadata!");
            Status::failed_precondition("Missing tenant on request metadata")
        })?;

        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or_else(|| {
            error!("Requestor not found on request metadata!");
            Status::failed_precondition("Missing requestor on request metadata")
        })?;

        info!(
            "{}",
            lib_data::log::format(
                lib_rpc::get_address(&req),
                &requestor,
                &tenant,
                "delete user"
            )
        );

        let result = {
            let (dbpool, rmqpool) = self.pools.get(&tenant).expect("Unable to find tenant");

            let rabbitmq = rmqpool.get().await.map_err(|e| {
                error!("Could not connect to RabbitMQ: {}", e);
                Status::internal("There was an error while trying to connect to the message broker")
            })?;

            repository::delete_user(req.get_ref().index, requestor, dbpool, &rabbitmq).await
        };

        result.map(|_| Response::new(()))
    }
}
