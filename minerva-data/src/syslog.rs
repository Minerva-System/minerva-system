//! This module contains Data Transfer Objects for the `syslog` table, which
//! represents the audit logs of the service.
//!
//! `syslog` holds information of insertions, updates and deletions from any
//! table, and also describes which service did it and which user of this given
//! tenance requested the operation.

use crate::enum_error::EnumError;
use crate::schema::syslog;
use chrono::{DateTime, Utc};
use diesel::sql_types::SmallInt;
use num_derive::{FromPrimitive, ToPrimitive};
use serde_repr::Serialize_repr;

/// Enumeration for the type of operation registered on `syslog`.
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

/// DTO representing a single entry on the `syslog` table.
#[derive(Queryable, Serialize, Clone, Debug, PartialEq)]
pub struct Syslog {
    /// ID of the entry on the table.
    pub id: i32,
    /// Name of the service that performed the operation.
    pub service: String,
    /// Username of whom requested the operation.
    pub requestor: String,
    /// Name of the entity that was manipulated, usually the same as a table.
    pub entity: String,
    /// Operation performed.
    pub operation: OpType,
    /// Date and time of the performed operation.
    pub datetime: DateTime<Utc>,
    /// Optional description of the performed operation.
    pub description: Option<String>,
}

/// DTO for adding a new entry on the `syslog` table.
#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "syslog"]
pub struct NewLog {
    /// Name of the service performing the operation.
    pub service: String,
    /// Username of whom is requesting the operation.
    pub requestor: String,
    /// Name of the entity being manipulated, usually the same as a table.
    pub entity: String,
    /// Operation being performed.
    pub operation: OpType,
    /// Date and time of the operation being performed.
    pub datetime: DateTime<Utc>,
    /// Optional description of the operation being performed.
    pub description: Option<String>,
}
