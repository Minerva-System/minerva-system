//! This controller contains routines related to dispatching session management
//! messages, normally to a SESSION module.

use crate::error::DispatchError;
use futures::stream::TryStreamExt;
use log::{debug, error, info, trace};
use minerva_broker::model::SessionMessage;
use minerva_rpc as rpc;
use mongodb::bson::{doc, Document};
use mongodb::Client as MongoClient;
use mongodb::Database as MongoDatabase;

/// Dispatches a user session message.
pub async fn dispatch(
    tenant: &str,
    consumer_name: &str,
    mongodb: &MongoClient,
    data: &[u8],
) -> Result<bool, Box<dyn std::error::Error>> {
    trace!("Session management dispatch");
    match SessionMessage::from(std::str::from_utf8(data).unwrap().to_string()) {
        SessionMessage::Remove { user } => {
            // `SESSION` already implements session removal, including
            // un-caching.
            // We should dispatch to that instead of relying on direct
            // database operations. It doesn't matter that we still have
            // access to connections to other services, since those
            // clients are asynchronous in nature and will only open
            // connections on demand.
            info!("{}: Remove session from user \"{}\"", consumer_name, user);
            remove_user_sessions(consumer_name, &mongodb.database(tenant), tenant, &user).await?;
            Ok(true)
        }
        _ => {
            // Unknown message
            debug!("Received message is unknown!");
            Ok(false)
        }
    }
}

/// Dispatches requests to SESSION so that all sessions for a given user are
/// removed.
async fn remove_user_sessions(
    consumer_name: &str,
    mongodb: &MongoDatabase,
    tenant: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Remove sessions for user {}@{}", username, tenant);
    use tonic::Request;

    // 1. Fetch all sessions from the given user
    let collection = mongodb.collection::<Document>("session");
    let filter = doc! { "login": username };
    let mut cursor = collection.find(filter, None).await?;

    let mut sessions = vec![];
    while let Some(document) = cursor.try_next().await.unwrap() {
        sessions.push(rpc::messages::SessionToken {
            token: base64::encode(
                document
                    .get("_id")
                    .unwrap()
                    .as_object_id()
                    .unwrap()
                    .to_hex(),
            ),
        });
    }

    info!("Removing {} sessions.", sessions.len());

    // 2. Ask `SESSION` to kill them all.
    let server = std::env::var("SESSION_SERVICE_SERVER")?;
    let port = std::env::var("SESSION_SERVICE_PORT")?;
    let endpoint = format!("http://{}:{}", server, port);

    debug!("Create client for service SESSION");
    let mut client = rpc::session::make_client(endpoint, tenant.into(), "DISPATCH".into())
        .await
        .map_err(|_| {
            let error = DispatchError::Connection {
                consumer_name: consumer_name.to_owned(),
                service_name: "SESSION service".to_string(),
            };
            error!("{}", error);
            error
        })?;

    debug!("Removing sessions");
    for session in sessions {
        client.remove(Request::new(session)).await?;
    }

    debug!("Sessions removed successfully");
    Ok(())
}
