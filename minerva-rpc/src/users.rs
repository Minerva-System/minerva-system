tonic::include_proto!("user");

use super::metadata::ClientInterceptor;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

pub type UsersInterceptedService =
    users_client::UsersClient<InterceptedService<Channel, ClientInterceptor>>;

pub async fn make_client(
    endpoint: String,
    tenant: String,
    requestor: String,
) -> UsersInterceptedService {
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
