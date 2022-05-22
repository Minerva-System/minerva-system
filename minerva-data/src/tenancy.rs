//! This module wraps functions and structures for manipulating multi-tenancy
//! configuration.
//!
//! Multi-tenancy configuration is expected to be in the `tenancy.toml` file,
//! at root of the running service.

use serde_derive::Deserialize;
use std::collections::HashMap;

/// Tenant data for a single entry of a table in the TOML file.
/// Each entry is prepended by a `[[tenants]]` line.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Tenant {
    /// Human-readable tenant name, mostly for aesthetic reasons.
    pub name: String,
    /// Actual tenant name and database name on any database service.
    pub database: String,
    /// Max number of connections on a connection pool, whenever it applies.
    pub connections: u32,
}

/// Get a list of tenant configuration from a given configuration file,
/// usually `tenancy.toml`.
pub fn get_tenants(filename: &str) -> Vec<Tenant> {
    let file = crate::file::slurp(filename);

    let map: HashMap<String, Vec<Tenant>> = toml::from_str(&file).unwrap();
    map["tenants"].clone()
}
