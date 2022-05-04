use minerva_data::{
    db, encryption,
    user::{InsertableUser, User},
};
use minerva_rpc::messages;
use minerva_rpc::users::users_server::Users;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::{Request, Response, Status};

#[derive(Default, Clone)]
pub struct UsersService;

fn get_login(map: &MetadataMap) -> String {
    map.get("login")
        .unwrap_or(&MetadataValue::from_str("unknown").unwrap())
        .to_str()
        .unwrap()
        .to_string()
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn index(&self, req: Request<()>) -> Result<Response<messages::UserList>, Status> {
        let user = get_login(req.metadata());
        unimplemented!();
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::User>, Status> {
        let user = get_login(req.metadata());
        unimplemented!();
    }

    async fn store(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let user = get_login(req.metadata());
        unimplemented!();
    }

    async fn update(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let user = get_login(req.metadata());
        unimplemented!();
    }
}
