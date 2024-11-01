use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::response::status::Unauthorized;

use crate::auth::{create_token, verify_pass};

#[derive(Deserialize)]
struct Credentials<'r> {
    password: &'r str
}

#[derive(Serialize)]
struct Token {
    token: String
}

#[post("/login", data="<task>")]
fn login(task: Json<Credentials<'_>>) -> Result<Json<Token>, Unauthorized<String>> {
    if verify_pass(task.password) {
        match create_token() {
            Ok(token) => Ok(Json(Token {token})),
            Err(error) => Err(Unauthorized(error))
        }
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