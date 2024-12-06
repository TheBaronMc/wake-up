use figment::Figment;
use log::{info, LevelFilter};
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
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Start");
    let configuration_result = load_configuration();
    if let Err(error) = configuration_result {
        error!("{}", error);
        return;
    }

    let wake_up_config = configuration_result.unwrap();
    let rocket_config =
        Figment::from(rocket::Config::figment()).merge(("port", wake_up_config.port()));

    let mut rocket = rocket::custom(rocket_config).attach(catchers::stage());

    if *wake_up_config.api_enabled() {
        rocket = rocket.attach(routes::pages::stage());
    }

    if *wake_up_config.web_enabled() {
        rocket = rocket
            .attach(routes::api::login::stage())
            .attach(routes::api::configuration::stage())
            .attach(routes::api::groups::stage())
            .attach(routes::api::hosts::stage());
    }

    rocket.launch().await.expect("Error while lauching rocket");

    ()
}
