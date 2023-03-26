use diesel::backend::Backend;
use diesel::query_builder::*;
use diesel::result::QueryResult;
use diesel::Connection;
use diesel::RunQueryDsl;

/// A Diesel statement for creating a database.
#[derive(Debug, Clone)]
pub struct CreateDatabaseStmt {
    dbname: String,
}

impl CreateDatabaseStmt {
    /// Creates a new `CREATE DATABASE` statement. Requires the
    /// name of the database to be created.
    pub fn new(dbname: &str) -> Self {
        Self {
            dbname: dbname.into(),
        }
    }
}

impl<DB: Backend> QueryFragment<DB> for CreateDatabaseStmt {
    fn walk_ast(&self, mut out: AstPass<'_, DB>) -> QueryResult<()> {
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

/// Runs a database creation query, if said database does not exist.
/// The database in question shall have the same name of a tenant.
/// This function also expects the endpoint to a database server
/// (e.g. `localhost:5432`).
pub fn create_database(tenant: &str, server: &str) -> Result<(), String> {
    use crate::db;
    if db::try_make_single_connection(tenant, server).is_err() {
        let url = db::build_database_string("postgres", server);
        let conn = diesel::PgConnection::establish(&url).map_err(|e| format!("{}", e))?;
        CreateDatabaseStmt::new(tenant)
            .execute(&conn)
            .map_err(|e| format!("{}", e))?;
    }
    Ok(())
}
