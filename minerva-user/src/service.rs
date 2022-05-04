use minerva_rpc::messages;
use minerva_rpc::users::users_server::Users;
use tonic::{Request, Response, Status};

#[derive(Default, Clone)]
pub struct UsersService;

#[tonic::async_trait]
impl Users for UsersService {
    async fn index(&self, req: Request<()>) -> Result<Response<messages::UserList>, Status> {
        unimplemented!();
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::User>, Status> {
        unimplemented!();
    }

    async fn store(
        &self,
        req: Request<messages::UserWrite>,
    ) -> Result<Response<messages::User>, Status> {
        unimplemented!();
    }

    async fn update(
        &self,
        req: Request<messages::UserWrite>,
    ) -> Result<Response<messages::User>, Status> {
        unimplemented!();
    }
}
