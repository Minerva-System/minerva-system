use diesel::prelude::*;
use minerva_data::db::DBPool;
use minerva_data::encryption;
use minerva_data::schema::syslog;
use minerva_data::session as model;
use minerva_data::syslog::{NewLog, OpType};
use minerva_data::user::User;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use tonic::Status;

pub async fn create_session(
    data: model::NewSession,
    pool: DBPool,
    mongo: Database,
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
    if !encryption::check_hash(&data.password, &usr.pwhash) {
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

    // Try writing session log. Outcome doesn't matter
    let _ = {
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
    };

    Ok(token)
}

pub async fn recover_session(token: String, mongo: Database) -> Result<model::Session, Status> {
    let collection = mongo.collection::<model::Session>("session");

    // Decode session token
    let id = base64::decode(token.as_bytes())
        .map_err(|_| Status::internal("Unable to decode session token"))?;
    let id =
        String::from_utf8(id).map_err(|_| Status::internal("Unable to decode session token"))?;
    let id =
        ObjectId::parse_str(&id).map_err(|_| Status::internal("Unable to decode session token"))?;

    // Find session object
    collection
        .find_one(doc! { "_id": id }, None)
        .await
        .map_err(|e| Status::internal(format!("Error while trying to recover session: {}", e)))?
        .ok_or_else(|| Status::not_found("Session does not exist"))
}

pub async fn remove_session(token: String, pool: DBPool, mongo: Database) -> Result<(), Status> {
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
        let _ = {
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
        };
    }

    // If session was not found, return success anyway
    Ok(())
}
