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
        todo!();
    }

    async fn retrieve(
        &self,
        req: Request<messages::SessionToken>,
    ) -> Result<Response<messages::SessionData>, Status> {
        todo!();
    }

    async fn remove(&self, req: Request<messages::SessionToken>) -> Result<Response<()>, Status> {
        todo!();
    }
}
