use rocket::http::Status;

use crate::{
    reader::load_configuration,
    routes::{errors::ApiError, guard::token::Token},
};

#[get("/reload")]
fn reload(_token: Token) -> Result<Status, ApiError> {
    match load_configuration() {
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
