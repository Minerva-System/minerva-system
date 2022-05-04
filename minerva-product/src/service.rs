use minerva_rpc::messages;
use minerva_rpc::products::products_server::Products;
use tonic::{Request, Response, Status};

#[derive(Default, Clone)]
pub struct ProductsService;

#[tonic::async_trait]
impl Products for ProductsService {
    async fn index(&self, req: Request<()>) -> Result<Response<messages::ProductList>, Status> {
        unimplemented!();
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::Product>, Status> {
        unimplemented!();
    }

    async fn store(
        &self,
        req: Request<messages::Product>,
    ) -> Result<Response<messages::Product>, Status> {
        unimplemented!();
    }

    async fn update(
        &self,
        req: Request<messages::Product>,
    ) -> Result<Response<messages::Product>, Status> {
        unimplemented!();
    }
}
