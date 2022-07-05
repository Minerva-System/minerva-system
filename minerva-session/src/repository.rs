//! This module wraps the repository which handles the session DTOs.

use diesel::prelude::*;
use minerva_cache as cache;
use minerva_data::db::DBPool;
use minerva_data::encryption;
use minerva_data::schema::syslog;
use minerva_data::session as model;
use minerva_data::syslog::{NewLog, OpType};
use minerva_data::user::User;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database as MongoDatabase;
use redis::Client as RedisClient;
use std::str;
use tonic::Status;

/// Creates a new session for a user. Given the data for a new session, checks
/// if the database contains that user, if the password matches and, if it does,
/// creates a new session on the non-relational database and returns its ID as a
/// Base64 encoded string that should be stored on a cookie.
pub async fn create_session(
    tenant: &str,
    data: model::NewSession,
    pool: DBPool,
    mongo: MongoDatabase,
    redis: &RedisClient,
) -> Result<String, Status> {
    let usr = {
        use minerva_data::schema::user::dsl::*;

        let connection = pool
            .get()
            .await
            .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

        user.filter(login.eq(data.login.clone()))
            .first::<User>(&*connection)
    }
    .map_err(|_| Status::unauthenticated("Invalid login or password"))?;

    // Check for correct password
    let pwhash = str::from_utf8(&usr.pwhash)
        .map_err(|e| Status::internal(format!("Error while performing authentication: {}", e)))?;
    if !encryption::check_hash(&data.password, pwhash) {
        return Err(Status::unauthenticated("Invalid login or password"));
    }

    // Create session object
    let collection = mongo.collection::<model::Session>("session");
    let session: model::Session = data.into();

    let result = collection
        .insert_one(session.clone(), None)
        .await
        .map_err(|e| Status::internal(format!("Error while creating session: {}", e)))?
        .inserted_id
        .as_object_id()
        .ok_or_else(|| Status::internal("Erro ao processar token de sessão"))?
        .to_hex();
    let token = base64::encode(result);

    // Write token to cache. Outcome doesn't matter at this point
    let _ = serde_json::to_string(&session)
        .map(|json| cache::auth::save_session(redis, tenant, &token, &json));

    // Try writing session log. Outcome doesn't matter
    let connection = pool
        .get()
        .await
        .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

    let _ = diesel::insert_into(syslog::table)
        .values(&NewLog {
            service: "SESSION".to_string(),
            requestor: session.login.clone(),
            entity: "session".to_string(),
            operation: OpType::Insert,
            datetime: chrono::offset::Utc::now(),
            description: Some(format!("Create user session. Token: {}", token)),
        })
        .execute(&*connection);

    Ok(token)
}

/// Recovers a user's session from the non-relational database, given a
/// previously generated token. The token must be the actual ID for the session
/// object on the non-relational database, encoded as Base64. If it was found,
/// returns the `Session` object with the session information that was stored.
pub async fn recover_session(
    tenant: &str,
    token: String,
    mongo: MongoDatabase,
    redis: &RedisClient,
) -> Result<model::Session, Status> {
    // Try to find object on Redis. If we do, return it
    if let Ok(json) = cache::auth::get_session(redis, tenant, &token).await {
        return serde_json::from_str(&json).map_err(|e| {
            Status::internal(format!(
                "Error while recovering session from cache: {:?}",
                e
            ))
        });
    }

    let collection = mongo.collection::<model::Session>("session");

    // Decode session token
    let id = base64::decode(token.as_bytes())
        .map_err(|_| Status::internal("Unable to decode session token"))?;
    let id =
        String::from_utf8(id).map_err(|_| Status::internal("Unable to decode session token"))?;
    let id =
        ObjectId::parse_str(&id).map_err(|_| Status::internal("Unable to decode session token"))?;

    // Otherwise, find session object on database
    let session = collection
        .find_one(doc! { "_id": id }, None)
        .await
        .map_err(|e| Status::internal(format!("Error while trying to recover session: {}", e)))?
        .ok_or_else(|| Status::not_found("Session does not exist"))?;

    // Now cache the session object on Redis, in JSON format.
    // When caching, we shouldn't really care about the outcome.
    // Just log stuff to console on error.
    if let Ok(json) = serde_json::to_string(&session) {
        if let Err(e) = cache::auth::save_session(redis, tenant, &token, &json) {
            println!("Error while caching recovered user session: {:#?}", e);
        }
    }

    Ok(session)
}

/// Removes a user session from non-relational database, given a session token.
/// The token must be the actual ID for the session object on the non-relational
/// database, encoded as Base64. If it was found, remove it altogether from the
/// non-relational database.
pub async fn remove_session(
    tenant: &str,
    token: String,
    pool: DBPool,
    mongo: MongoDatabase,
    redis: &RedisClient,
) -> Result<(), Status> {
    // Remove session from cache, forcibly.
    cache::auth::remove_session(redis, tenant, &token)
        .await
        .map_err(|e| Status::internal(format!("Could not remove session from cache: {:?}", e)))?;

    let collection = mongo.collection::<model::Session>("session");

    // Decode session token
    let id = base64::decode(token.as_bytes())
        .map_err(|_| Status::internal("Unable to decode session token"))?;
    let id =
        String::from_utf8(id).map_err(|_| Status::internal("Unable to decode session token"))?;
    let id =
        ObjectId::parse_str(&id).map_err(|_| Status::internal("Unable to decode session token"))?;

    // Find session object
    if let Ok(session) = collection.find_one(doc! { "_id": id }, None).await {
        // Since the object was found, remove it
        collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| Status::internal(format!("Error while deleting session data: {}", e)))?;

        // Try writing session log. Outcome doesn't matter
        let connection = pool
            .get()
            .await
            .map_err(|e| Status::internal(format!("Database access error: {}", e)))?;

        let _ = diesel::insert_into(syslog::table)
            .values(&NewLog {
                service: "SESSION".to_string(),
                requestor: session
                    .map(|s| s.login)
                    .unwrap_or_else(|| "unknown".to_string()),
                entity: "session".to_string(),
                operation: OpType::Delete,
                datetime: chrono::offset::Utc::now(),
                description: Some(format!("Remove user session. Token: {}", token)),
            })
            .execute(&*connection);
    }

    // If session was not found, return success anyway
    Ok(())
}
