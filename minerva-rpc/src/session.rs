//! Structures and algorithms related to a server and a client for the Session
//! service.

tonic::include_proto!("session");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

/// Type for a `SessionClient` with an interceptor that sends tenant and
/// requestor metadata to the server.
pub type SessionInterceptedClient =
    session_client::SessionClient<InterceptedService<Channel, ClientInterceptor>>;

/// Creates a Session client with tenant and requestor metadata and connects
/// to the server.
///
/// This function requires information about `tenant`, `requestor` and the
/// actual `endpoint` that the client should connect to.
pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> SessionInterceptedClient {
    let channel = Channel::from_shared(endpoint.clone())
        .unwrap()
        .connect()
        .await
        .unwrap();
    session_client::SessionClient::with_interceptor(
        channel,
        ClientInterceptor::new(&tenant, &requestor),
    )
}
