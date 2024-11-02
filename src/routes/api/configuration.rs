use rocket::http::Status;

use crate::{configuration::{read_configuration, CONFIGURATION_PATH}, routes::guard::token::Token};

#[get("/reload")]
fn reload(_token: Token) -> Status {
    match read_configuration(CONFIGURATION_PATH) {
        Ok(_) => Status::Ok,
        Err(_error) => Status::NotModified
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CONFIGURATION", |rocket| async {
        rocket
        .mount("/api/configuration", routes![reload])
    })
}