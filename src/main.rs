#[macro_use] extern crate rocket;

mod routes;
mod auth;
mod host;
mod group;
mod configuration;
mod wol;

use crate::configuration::{CONFIGURATION_PATH, read_configuration};

#[rocket::main]
async fn main() -> () {
    if let Err(error) = read_configuration(CONFIGURATION_PATH) {
        println!("{}", error);
        return
    }

    let _rocket = rocket::build()
        .attach(routes::pages::stage())
        .attach(routes::api::login::stage())
        .attach(routes::api::configuration::stage())
        .attach(routes::api::groups::stage())
        .attach(routes::api::hosts::stage())
        .launch()
        .await
        .expect("Error while lauching rocket");
    
    ()
}