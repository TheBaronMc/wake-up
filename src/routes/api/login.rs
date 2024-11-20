use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::auth::{create_token, verify_pass};
use crate::routes::errors::ApiError;

#[derive(Deserialize)]
struct Credentials<'r> {
    password: &'r str,
}

#[derive(Serialize)]
struct AuthentificationSuccessBody {
    token: String,
}

#[post("/login", data = "<task>")]
fn login(task: Json<Credentials<'_>>) -> Result<Json<AuthentificationSuccessBody>, ApiError> {
    if verify_pass(task.password) {
        match create_token() {
            Ok(token) => Ok(Json(AuthentificationSuccessBody { token })),
            Err(error) => Err(ApiError::unauthorized(Some(error))),
        }
    } else {
        Err(ApiError::unauthorized(Some(String::from("Wrong password"))))
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("AUTH", |rocket| async {
        rocket.mount("/api", routes![login])
    })
}
