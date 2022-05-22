#![warn(clippy::all)]
#![warn(missing_docs)]

#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod controller;
mod fairings;
mod utils;

#[launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    println!("Minerva System: REST service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    rocket::build()
        .mount("/", controller::auth::routes())
        .mount("/users", controller::user::routes())
}
