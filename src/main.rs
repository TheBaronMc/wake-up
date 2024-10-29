#[macro_use] extern crate rocket;

mod api;
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
        .attach(api::pages::stage())
        .attach(api::auth::stage())
        .attach(api::configuration::stage())
        .launch()
        .await
        .expect("Error while lauching rocket");
    
    ()
}