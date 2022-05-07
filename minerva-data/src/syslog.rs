use crate::enum_error::EnumError;
use crate::schema::syslog;
use chrono::{DateTime, Utc};
use diesel::sql_types::SmallInt;
use num_derive::{FromPrimitive, ToPrimitive};
use serde_repr::Serialize_repr;

#[derive(
    FromPrimitive,
    ToPrimitive,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    AsExpression,
    FromSqlRow,
    DbEnum,
    Serialize_repr,
)]
#[sql_type = "SmallInt"]
#[error_fn = "EnumError::precondition_failed"]
#[error_type = "EnumError"]
#[repr(i16)]
pub enum OpType {
    Insert = 0,
    Update = 1,
    Delete = 2,
}

#[derive(Queryable, Serialize, Clone, Debug, PartialEq)]
pub struct Syslog {
    pub id: i32,
    pub service: String,
    pub requestor: String,
    pub entity: String,
    pub operation: OpType,
    pub datetime: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "syslog"]
pub struct NewLog {
    pub service: String,
    pub requestor: String,
    pub entity: String,
    pub operation: OpType,
    pub datetime: DateTime<Utc>,
    pub description: Option<String>,
}
