use configuration::read_global_configuration;
use figment::Figment;
use log::LevelFilter;
use reader::load_configuration;
use rocket::{Build, Rocket};
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
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    if let Err(error) = load_configuration() {
        panic!("Error while first loading configuration: {}", error)
    }

    let rocket: Rocket<Build> = read_global_configuration(|global_configuration| {
        let configuration =
            global_configuration.expect("Error while reading configuration at launch time");

        let rocket_config =
            Figment::from(rocket::Config::figment()).merge(("port", configuration.port()));

        let mut rocket = rocket::custom(rocket_config).attach(catchers::stage(
            *configuration.web_enabled(),
            *configuration.api_enabled(),
        ));

        if *configuration.web_enabled() {
            rocket = rocket.attach(routes::pages::stage());
        }

        if *configuration.api_enabled() {
            rocket = rocket
                .attach(routes::api::login::stage())
                .attach(routes::api::configuration::stage())
                .attach(routes::api::hosts::stage())
                .attach(routes::api::groups::stage());
        }

        rocket
    });

    rocket.launch().await.expect("Error while lauching rocket");

    ()
}
