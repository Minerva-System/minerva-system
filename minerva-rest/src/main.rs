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

    rocket::build()
        .register("/", controller::handlers::catchers())
        .mount("/", controller::auth::routes())
        .mount("/<_>/user", controller::user::routes())
}
