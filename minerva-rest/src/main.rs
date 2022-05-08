#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod routes;

#[launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    println!("Minerva System: REST service");
    println!("Copyright (c) 2022 Lucas S. Vieira");
    println!();

    dotenv().ok();

    rocket::build().mount("/", routes::user::routes())
}
