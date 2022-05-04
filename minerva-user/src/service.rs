use minerva_data::db::DBPool;
use minerva_data::{
    encryption,
    user::{InsertableUser, User},
};
use minerva_rpc::messages;
use minerva_rpc::users::users_server::Users;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct UsersService {
    pub pool: DBPool,
}

fn get_login(map: &MetadataMap) -> String {
    map.get("login")
        .unwrap_or(&MetadataValue::from_str("unknown").unwrap())
        .to_str()
        .unwrap()
        .to_string()
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn index(&self, _req: Request<()>) -> Result<Response<messages::UserList>, Status> {
        let users = {
            use diesel::prelude::*;
            use minerva_data::schema::user::dsl::*;

            let connection = self
                .pool
                .get()
                .await
                .map_err(|_| Status::internal("Database access error"))?;

            user.order(id)
                .limit(100)
                .offset(0)
                .load::<User>(&*connection)
                .map_err(|_| Status::internal("Cannot recover users"))?
        };

        Ok(Response::new(minerva_data::user::vec_to_message(users)))
    }

    async fn show(
        &self,
        req: Request<messages::EntityIndex>,
    ) -> Result<Response<messages::User>, Status> {
        let user_id = req.get_ref().index;
        let user = {
            use diesel::prelude::*;
            use minerva_data::schema::user::dsl::*;

            let connection = self
                .pool
                .get()
                .await
                .map_err(|_| Status::internal("Database access error"))?;

            user.filter(id.eq(user_id))
                .first::<User>(&*connection)
                .optional()
                .map_err(|_| Status::internal("Cannot recover user"))?
        };

        if let Some(usr) = user {
            Ok(Response::new(usr.into()))
        } else {
            Err(Status::not_found(format!("User ID {} not found.", user_id)))
        }
    }

    async fn store(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let login = get_login(req.metadata());
        unimplemented!();
    }

    async fn update(
        &self,
        req: Request<messages::User>,
    ) -> Result<Response<messages::User>, Status> {
        let login = get_login(req.metadata());
        unimplemented!();
    }
}
