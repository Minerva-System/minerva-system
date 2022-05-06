use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Tenant {
    pub name: String,
    pub database: String,
}

pub fn get_tenants(filename: &str) -> Vec<Tenant> {
    let file = crate::file::slurp(filename);

    let map: HashMap<String, Vec<Tenant>> = toml::from_str(&file).unwrap();
    map["tenants"].clone()
}
