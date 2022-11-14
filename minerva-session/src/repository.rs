//! This module wraps the repository which handles the session DTOs.

use diesel::prelude::*;
use log::{debug, error, trace};
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
    trace!("Create new session");
    let usr = {
        use minerva_data::schema::user::dsl::*;

        let connection = pool.get().await.map_err(|e| {
            error!("Database access error: {}", e);
            Status::internal("There was an error while trying to access the database")
        })?;

        user.filter(login.eq(data.login.clone()))
            .first::<User>(&*connection)
    }
    .map_err(|_| {
        debug!("Invalid login or password");
        Status::unauthenticated("Invalid login or password")
    })?;

    // Check for correct password
    let pwhash = str::from_utf8(&usr.pwhash).map_err(|e| {
        debug!("Error while performing authentication: {}", e);
        Status::internal("There was an error while performing authentication")
    })?;

    if !encryption::check_hash(&data.password, pwhash) {
        return Err(Status::unauthenticated("Invalid login or password"));
    }

    // Create session object
    let collection = mongo.collection::<model::Session>("session");
    let session: model::Session = data.into();

    let result = collection
        .insert_one(session.clone(), None)
        .await
        .map_err(|e| {
            error!("Error while creating session: {}", e);
            Status::internal("There was an error while creating a new session")
        })?
        .inserted_id
        .as_object_id()
        .ok_or_else(|| {
            error!("Error while processing session token");
            Status::internal("Error while processing session token")
        })?
        .to_hex();
    let token = base64::encode(result);

    // Write token to cache. Outcome doesn't matter at this point
    let _ = serde_json::to_string(&session)
        .map(|json| cache::auth::save_session(redis, tenant, &token, &json));

    // Try writing session log. Outcome doesn't matter
    let connection = pool.get().await.map_err(|e| {
        error!("Error while acessing database for logging: {}", e);
        Status::internal("There was an error while trying to access the database")
    })?;

    let _ = diesel::insert_into(syslog::table)
        .values(&NewLog {
            service: "SESSION".to_string(),
            requestor: session.login.clone(),
            entity: "session".to_string(),
            operation: OpType::Insert,
            datetime: chrono::offset::Utc::now(),
            description: Some("Create user session".to_string()),
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
    trace!("Recover session");
    // Try to find object on Redis. If we do, return it
    if let Ok(json) = cache::auth::get_session(redis, tenant, &token).await {
        return serde_json::from_str(&json).map_err(|e| {
            error!(
                "Error while parsing session data from Redis as JSON: {:?}",
                e
            );
            Status::internal("There was an error while recovering a cached session")
        });
    }

    let collection = mongo.collection::<model::Session>("session");

    // Decode session token
    let id = decode_session_token(token.clone())?;

    // Otherwise, find session object on database
    let session = collection
        .find_one(doc! { "_id": id }, None)
        .await
        .map_err(|e| {
            error!("Could not recover session token from MongoDB: {}", e);
            Status::internal("Error while trying to recover session")
        })?
        .ok_or_else(|| Status::not_found("Session does not exist"))?;

    // Now cache the session object on Redis, in JSON format.
    // When caching, we shouldn't really care about the outcome.
    // Just log stuff to console on error.
    if let Ok(json) = serde_json::to_string(&session) {
        if let Err(e) = cache::auth::save_session(redis, tenant, &token, &json) {
            error!("Error while caching recovered user session: {:#?}", e);
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
    trace!("Removing user session");
    // Remove session from cache, forcibly.
    cache::auth::remove_session(redis, tenant, &token)
        .await
        .map_err(|e| {
            error!("Could not remove session from cache: {:?}", e);
            Status::internal("Could not remove session from cache")
        })?;

    let collection = mongo.collection::<model::Session>("session");

    // Decode session token
    let id = decode_session_token(token.clone())?;

    // Find session object
    if let Ok(session) = collection.find_one(doc! { "_id": id }, None).await {
        // Since the object was found, remove it
        collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(|e| {
                error!("Error while deleting session data: {}", e);
                Status::internal("There was an error while trying to delete the session")
            })?;

        // Try writing session log. Outcome doesn't matter
        let connection = pool.get().await.map_err(|e| {
            error!("Error while accessing database for logging: {}", e);
            Status::internal("There was an error while trying to access the database")
        })?;

        let _ = diesel::insert_into(syslog::table)
            .values(&NewLog {
                service: "SESSION".to_string(),
                requestor: session
                    .map(|s| s.login)
                    .unwrap_or_else(|| "unknown".to_string()),
                entity: "session".to_string(),
                operation: OpType::Delete,
                datetime: chrono::offset::Utc::now(),
                description: Some("Remove user session".to_string()),
            })
            .execute(&*connection);
    }

    // If session was not found, return success anyway
    Ok(())
}

/// Takes a Base64 encoded session token and transforms it back into
/// a MongoDB ObjectId, if possible.
fn decode_session_token(token: String) -> Result<ObjectId, Status> {
    let id = base64::decode(token.as_bytes()).map_err(|_| {
        error!("Could not decode session token from Base64");
        Status::internal("Unable to decode session token")
    })?;

    let id = String::from_utf8(id).map_err(|_| {
        error!("Could not convert ID to UTF-8");
        Status::internal("Unable to decode session token")
    })?;

    ObjectId::parse_str(&id).map_err(|_| {
        error!("Could not parse session token as MongoDB Object ID");
        Status::internal("Unable to decode session token")
    })
}
