use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::response::status::Unauthorized;

use crate::configuration::CONFIGURATION;

#[derive(Deserialize)]
struct Credentials<'r> {
    password: &'r str
}

#[derive(Serialize)]
struct Token<'r> {
    token: &'r str
}

#[post("/login", data="<task>")]
fn login(task: Json<Credentials<'_>>) -> Result<Json<Token>, Unauthorized<String>> {
    let current_configuration = CONFIGURATION.read().expect("Failed to read configuration");

    if task.password == current_configuration.password {
        Ok(Json(Token {
            token: "toto"
        }))
    } else {
        Err(Unauthorized(String::from("Wrong password")))
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("AUTH", |rocket| async {
        rocket
        .mount("/api", routes![login])
    })
}