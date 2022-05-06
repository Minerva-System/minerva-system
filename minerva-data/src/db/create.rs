use diesel::backend::Backend;
use diesel::query_builder::*;
use diesel::result::QueryResult;
use diesel::Connection;
use diesel::RunQueryDsl;

#[derive(Debug, Clone)]
pub struct CreateDatabaseStmt {
    dbname: String,
}

impl CreateDatabaseStmt {
    pub fn new(dbname: &str) -> Self {
        Self {
            dbname: dbname.into(),
        }
    }
}

impl<DB: Backend> QueryFragment<DB> for CreateDatabaseStmt {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, DB>) -> QueryResult<()> {
        out.push_sql("CREATE DATABASE ");
        out.push_identifier(&self.dbname)?;
        Ok(())
    }
}

impl<Conn> RunQueryDsl<Conn> for CreateDatabaseStmt {}

impl QueryId for CreateDatabaseStmt {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

pub fn create_database(tenant: &str) -> Result<(), String> {
    use crate::db;
    if db::try_make_single_connection(tenant).is_err() {
        println!("{}: Creating database...", tenant);
        let url = db::build_database_string("postgres");
        let conn = diesel::PgConnection::establish(&url).map_err(|e| format!("{}", e))?;
        CreateDatabaseStmt::new(tenant)
            .execute(&conn)
            .map_err(|e| format!("{}", e))?;
    }
    Ok(())
}
