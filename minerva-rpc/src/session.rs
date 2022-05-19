tonic::include_proto!("session");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

pub type SessionInterceptedService =
    session_client::SessionClient<InterceptedService<Channel, ClientInterceptor>>;

pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> SessionInterceptedService {
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
