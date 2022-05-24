//! Structures and algorithms related to a server and a client for the Products
//! service.

tonic::include_proto!("product");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

/// Type for a `ProductsClient` with an interceptor that sends tenant and
/// requestor metadata to the server.
pub type ProductsInterceptedClient =
    products_client::ProductsClient<InterceptedService<Channel, ClientInterceptor>>;

/// Creates a Products client with tenant and requestor metadata and connects
/// to the server.
///
/// This function requires information about `tenant`, `requestor` and the
/// actual `endpoint` that the client should connect to.
pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> ProductsInterceptedClient {
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
