//! Structures and algorithms related to a server and a client for the Users
//! service.

tonic::include_proto!("user");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

/// Type for a `UsersClient` with an interceptor that sends tenant and
/// requestor metadata to the server.
pub type UsersInterceptedClient =
    users_client::UsersClient<InterceptedService<Channel, ClientInterceptor>>;

/// Creates a Users client with tenant and requestor metadata and connects
/// to the server.
///
/// This function requires information about `tenant`, `requestor` and the
/// actual `endpoint` that the client should connect to.
pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> UsersInterceptedClient {
    let channel = Channel::from_shared(endpoint.clone())
        .unwrap()
        .connect()
        .await
        .unwrap();
    users_client::UsersClient::with_interceptor(
        channel,
        ClientInterceptor::new(&tenant, &requestor),
    )
}
