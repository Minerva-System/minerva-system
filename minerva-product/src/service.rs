use minerva_data as lib_data;
use minerva_rpc as lib_rpc;
use minerva_rpc::messages;
use minerva_rpc::metadata;
use minerva_rpc::products::products_server::Products;
use tonic::{Request, Response, Status};

#[derive(Default, Clone)]
pub struct ProductsService;

#[tonic::async_trait]
impl Products for ProductsService {
    async fn index(&self, req: Request<()>) -> Result<Response<messages::ProductList>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or(
            Status::failed_precondition("Missing tenant on request metadata"),
        )?;
        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or(
            Status::failed_precondition("Missing requestor on request metadata"),
        )?;
        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "PRODUCT::INDEX",
        );

        unimplemented!();
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::Product>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or(
            Status::failed_precondition("Missing tenant on request metadata"),
        )?;
        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or(
            Status::failed_precondition("Missing requestor on request metadata"),
        )?;
        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "PRODUCT::SHOW",
        );

        unimplemented!();
    }

    async fn store(
        &self,
        req: Request<messages::Product>,
    ) -> Result<Response<messages::Product>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or(
            Status::failed_precondition("Missing tenant on request metadata"),
        )?;
        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or(
            Status::failed_precondition("Missing requestor on request metadata"),
        )?;
        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "PRODUCT::STORE",
        );

        unimplemented!();
    }

    async fn update(
        &self,
        req: Request<messages::Product>,
    ) -> Result<Response<messages::Product>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or(
            Status::failed_precondition("Missing tenant on request metadata"),
        )?;
        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or(
            Status::failed_precondition("Missing requestor on request metadata"),
        )?;
        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "PRODUCT::UPDATE",
        );

        unimplemented!();
    }

    async fn delete(&self, req: Request<messages::EntityIndex>) -> Result<Response<()>, Status> {
        let tenant = metadata::get_value(req.metadata(), "tenant").ok_or(
            Status::failed_precondition("Missing tenant on request metadata"),
        )?;
        let requestor = metadata::get_value(req.metadata(), "requestor").ok_or(
            Status::failed_precondition("Missing requestor on request metadata"),
        )?;
        lib_data::log::print(
            lib_rpc::get_address(&req),
            requestor.clone(),
            tenant.clone(),
            "PRODUCT::DELETE",
        );

        unimplemented!();
    }
}
