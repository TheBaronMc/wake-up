use figment::Figment;
use reader::load_configuration;
use routes::catchers;

#[macro_use]
extern crate rocket;

mod auth;
mod configuration;
mod group;
mod host;
mod reader;
mod routes;
mod wol;

#[rocket::main]
async fn main() -> () {
    let configuration_result = load_configuration();
    if let Err(error) = configuration_result {
        println!("{}", error);
        return;
    }

    let wake_up_config = configuration_result.unwrap();
    let rocket_config =
        Figment::from(rocket::Config::figment()).merge(("port", wake_up_config.port()));

    let _rocket = rocket::custom(rocket_config)
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
