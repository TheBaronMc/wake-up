use rocket::http::Status;

use crate::{
    configuration::{read_configuration, CONFIGURATION_PATH},
    routes::{errors::ApiError, guard::token::Token},
};

#[get("/reload")]
fn reload(_token: Token) -> Result<Status, ApiError> {
    match read_configuration(CONFIGURATION_PATH) {
        Ok(_) => Ok(Status::Ok),
        Err(error) => {
            println!("[API] Failed to reload configuration {}", error);
            Err(ApiError::custom(Status::BadRequest, Some(error)))
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CONFIGURATION", |rocket| async {
        rocket.mount("/api/configuration", routes![reload])
    })
}
