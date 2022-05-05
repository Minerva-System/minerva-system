use crate::schema::syslog;
use chrono::{DateTime, Utc};

// 0 = inserção
// 1 = alteração
// 2 = remoção

#[derive(Queryable, Serialize, Clone, Debug, PartialEq)]
pub struct Syslog {
    pub id: i32,
    pub service: String,
    pub requestor: String,
    pub entity: String,
    pub operation: i16,
    pub datetime: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "syslog"]
pub struct NewLog {
    pub service: String,
    pub requestor: String,
    pub entity: String,
    pub operation: i16,
    pub datetime: DateTime<Utc>,
    pub description: Option<String>,
}
