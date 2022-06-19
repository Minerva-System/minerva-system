//! Structures and algorithms related to a server and a client for the Users
//! service.

tonic::include_proto!("user");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

/// Type for a `UsersClient` with an interceptor that sends tenant and
/// requestor metadata to the server.
pub type UsersInterceptedClient =
    users_client::UsersClient<InterceptedService<Channel, ClientInterceptor>>;

/// Creates a Users client with tenant and requestor metadata and connects
/// to the server. Upon failure, returns an `Unavailable` gRPC status.
///
/// This function requires information about `tenant`, `requestor` and the
/// actual `endpoint` that the client should connect to.
pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> Result<UsersInterceptedClient, Status> {
    Channel::from_shared(endpoint.clone())
        .unwrap()
        .connect()
        .await
        .map(|channel| {
            users_client::UsersClient::with_interceptor(
                channel,
                ClientInterceptor::new(&tenant, &requestor),
            )
        })
        .map_err(|e| Status::unavailable(format!("Error connecting to USERS service: {}", e)))
}
