//! # Minerva System: REST Service
//!
//! ## About this service
//! This service's responsibility is that of being a REST gateway for the rest
//! of the system. In other words, anything that can be accessed by the user
//! should be accessible through REST requests that are managed by this service.
//!
//! This service also should never manage entities in the database directly.
//! The REST service should always communicate to whatever gRPC service manages
//! the entity required by the remote user instead.

#![warn(clippy::all)]
#![warn(missing_docs)]

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::{
    mount_endpoints_and_merged_docs,
    rapidoc::*,
    settings::{OpenApiSettings, UrlObject},
    swagger_ui::*,
};
use std::env;

mod controller;
mod fairings;
mod utils;

#[cfg(test)]
mod tests;

/// Entry point for this service. Creates the service and routes that will be
/// served by the REST server.
#[launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    println!("Minerva System: REST service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    let logconfig = env::var("LOG_CONFIG_FILE").unwrap_or_else(|_| "./logging.yml".to_owned());
    log4rs::init_file(logconfig, Default::default()).expect("Could not initialize logs");

    let swagger_config = SwaggerUIConfig {
        url: "../openapi.json".to_owned(),
        ..Default::default()
    };

    let rapidoc_config = RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut building_rocket = rocket::build()
        .register("/", controller::handlers::catchers())
        .mount("/swagger", make_swagger_ui(&swagger_config))
        .mount("/rapidoc", make_rapidoc(&rapidoc_config));

    let openapi_settings = OpenApiSettings::default();

    mount_endpoints_and_merged_docs! {
    building_rocket, "/", openapi_settings,
    "" => controller::auth::routes(),
    "" => controller::user::routes(),
    };

    building_rocket
}
