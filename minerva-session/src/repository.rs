use diesel::prelude::*;
use minerva_data::db::DBPool;
use minerva_data::encryption;
use minerva_data::session as model;
use minerva_data::user::User;
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
    println!("Emplacing collection");
    let result = collection
        .insert_one(session, None)
        .await
        .map_err(|e| Status::internal(format!("Error while creating session: {}", e)))?
        .inserted_id
        .as_object_id()
        .ok_or_else(|| Status::internal("Erro ao processar token de sessão"))?
        .to_hex();
    let token = base64::encode(result);

    Ok(token)
}
