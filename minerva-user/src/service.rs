use crate::repository;
use minerva_data::db::DBPool;
use minerva_rpc::messages;
use minerva_rpc::users::users_server::Users;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct UsersService {
    pub pool: DBPool,
}

fn get_login(map: &MetadataMap) -> String {
    map.get("login")
        .unwrap_or(&MetadataValue::from_str("unknown").unwrap())
        .to_str()
        .unwrap()
        .to_string()
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn index(&self, _req: Request<()>) -> Result<Response<messages::UserList>, Status> {
        let result = {
            let connection = self
                .pool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::get_list(0, &*connection)
                .map_err(|e| Status::internal(format!("Cannot recover user list: {}", e)))?
        };

        Ok(Response::new(minerva_data::user::vec_to_message(result)))
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::User>, Status> {
        let result = {
            let connection = self
                .pool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::get_user(req.get_ref().index, &*connection)
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
        let requestor = get_login(req.metadata());
        let result = {
            let data = req.into_inner().into();

            let connection = self
                .pool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::add_user(data, requestor, &*connection)
        };

        result
            .map(|u| Response::new(u.into()))
            .map_err(|e| Status::failed_precondition(format!("Unable to register user: {}", e)))
    }

    async fn update(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let requestor = get_login(req.metadata());
        let result = {
            let data = req.into_inner().into();

            let connection = self
                .pool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::update_user(data, requestor, &*connection)
        };

        result
            .map(|u| Response::new(u.into()))
            .map_err(|e| Status::failed_precondition(format!("Unable to register user: {}", e)))
    }

    async fn delete(&self, req: Request<messages::EntityIndex>) -> Result<Response<()>, Status> {
        let requestor = get_login(req.metadata());
        let result = {
            let connection = self
                .pool
                .get()
                .await
                .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

            repository::delete_user(req.get_ref().index, requestor, &*connection)
        };

        result
            .map(|_| Response::new(()))
            .map_err(|e| Status::internal(format!("Cannot recover user: {}", e)))
    }
}
