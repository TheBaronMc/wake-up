use rocket::http::Status;

use crate::{
    reader::load_configuration,
    routes::{errors::ApiError, guard::token::Token},
};

#[get("/reload")]
fn reload(authorization: Result<Token, ApiError>) -> Result<Status, ApiError> {
    authorization?;

    load_configuration().or_else(|error| {
        println!("[Configuration] Failed to reload {}", error);
        Err(ApiError::custom(Status::BadRequest, Some(error)))
    })?;

    println!("[Configuration] Realoaded.");
    Ok(Status::Ok)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CONFIGURATION", |rocket| async {
        rocket.mount("/api/configuration", routes![reload])
    })
}
