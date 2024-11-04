#[macro_use]
extern crate rocket;

mod auth;
mod configuration;
mod group;
mod host;
mod routes;
mod wol;

use routes::catchers;

use crate::configuration::{read_configuration, CONFIGURATION_PATH};

#[rocket::main]
async fn main() -> () {
    if let Err(error) = read_configuration(CONFIGURATION_PATH) {
        println!("{}", error);
        return;
    }

    let _rocket = rocket::build()
        .attach(routes::pages::stage())
        .attach(routes::api::login::stage())
        .attach(routes::api::configuration::stage())
        .attach(routes::api::groups::stage())
        .attach(routes::api::hosts::stage())
        .attach(catchers::stage())
        .launch()
        .await
        .expect("Error while lauching rocket");

    ()
}
