tonic::include_proto!("product");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

pub type ProductsInterceptedService =
    products_client::ProductsClient<InterceptedService<Channel, ClientInterceptor>>;

pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> ProductsInterceptedService {
    let channel = Channel::from_shared(endpoint.clone())
        .unwrap()
        .connect()
        .await
        .unwrap();
    products_client::ProductsClient::with_interceptor(
        channel,
        ClientInterceptor::new(&tenant, &requestor),
    )
}
