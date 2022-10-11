//! This module contains Data Transfer Objects for the `syslog` table, which
//! represents the audit logs of the service.
//!
//! `syslog` holds information of insertions, updates and deletions from any
//! table, and also describes which service did it and which user of this given
//! tenance requested the operation.
#![allow(clippy::extra_unused_lifetimes)]

use crate::schema::syslog;
use chrono::{DateTime, Utc};

/// Enumeration for the type of operation registered on `syslog`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize)]
#[DieselType = "Op_type"]
pub enum OpType {
    /// Operation for inserting new entities.
    Insert,
    /// Operation for updating entities.
    Update,
    /// Operation for removing entities.
    Delete,
}

/// DTO representing a single entry on the `syslog` table.
#[derive(Queryable, Serialize, Clone, Debug, PartialEq, Eq)]
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
#[derive(Insertable, Clone, Debug, PartialEq, Eq)]
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
