//! Structures for handling gRPC request metadata.
//!
//! This module has structures and functions for extracting and inserting
//! metadata on gRPC requests.

use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

/// Retrieve the string value from a request's metadata map by its key.
///
/// # Example
///
/// ```ignore
/// let tenant = get_value(request.metadata(), "tenant")
///     .expect("Missing tenant info on request metadata");
/// ```
pub fn get_value(map: &MetadataMap, key: &str) -> Option<String> {
    map.get(key).map(|v| v.to_str().unwrap_or("unknown").into())
}

/// Emplace tenant and requestor information on a request's metadata.
///
/// Returns a string indicating the kind of error if it fails.
pub fn push_metadata(
    request: &mut Request<()>,
    tenant: &str,
    requestor: &str,
) -> Result<(), String> {
    request.metadata_mut().insert(
        "tenant",
        MetadataValue::try_from(tenant)
            .map_err(|e| format!("Failure while applying tenant to request: {}", e))?,
    );

    request.metadata_mut().insert(
        "requestor",
        MetadataValue::try_from(requestor)
            .map_err(|e| format!("Failure while applying requestor to request: {}", e))?,
    );

    Ok(())
}

/// Defines the interceptor data for any gRPC client.
///
/// The interceptor exists for the sole purpose of introducing `tenant` and
/// `requestor` data within a given request, that is going to be sent by the
/// client connected to a service.
///
/// # Example
///
/// Consider the following dummy service that allows a remote procedure call
/// to a `ping` function.
///
/// ```proto
/// // example.proto
///
/// syntax = "proto3";
/// import "google/protobuf/empty.proto";
///
/// package Example;
///
/// service Example {
///   rpc ping(google.protobuf.Empty) returns (google.protobuf.Empty) {}
/// }
/// ```
///
/// Imagine that this single procedure implementation requires that tenant and
/// requestor data are embedded within a request for this to work. For example:
///
/// ```ignore
/// // example.rs
/// tonic::include_proto!("example");
///
/// #[derive(Clone)]
/// pub struct ExampleService;
///
/// #[tonic::async_trait]
/// impl Example for ExampleService {
///     async fn ping(&self, req: Request<()>) -> Result<(), Status> {
///         let tenant = metadata::get_value(req.metadata(), "tenant")
///             .ok_or_else(|| Status::failed_precondition("Missing tenant data"))?;
///         let requestor = metadata::get_value(req.metadata(), "requestor")
///             .ok_or_else(|| Status::failed_precondition("Missing requestor data"))?;
///         println!("Tenant: {}, requestor: {}", tenant, requestor);
///         Ok(Response::new(()))
///     }
/// }
/// ```
///
/// One may want to use this structure to send data as needed.
///
/// ```ignore
/// // example.rs
/// fn test_request() {
///     use tonic::transport::Channel;
///     let channel = Channel::from_static("http://localhost:1234").connect().await?;
///     let client = test_client::TestClient::with_interceptor(
///         channel,
///         ClientInterceptor::new("company", "admin"),
///     );
///     let response = client.ping(Request::new(())).await?;
/// }
/// ```
pub struct ClientInterceptor {
    /// Tenant related to the database access while on a request.
    tenant: String,
    /// Requestor of the operation, usually the login of a user.
    requestor: String,
}

impl ClientInterceptor {
    /// Creates a new `ClientInterceptor` from given tenant and requestor
    /// names. The tenant is closely related to the database to be manipulated,
    /// while the requestor usually is the login of the user performing the
    /// manipulation.
    pub fn new(tenant: &str, requestor: &str) -> Self {
        Self {
            tenant: tenant.trim().into(),
            requestor: requestor.trim().into(),
        }
    }
}

impl Interceptor for ClientInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        push_metadata(&mut request, self.tenant.as_ref(), self.requestor.as_ref())
            .map_err(Status::failed_precondition)?;
        Ok(request)
    }
}
