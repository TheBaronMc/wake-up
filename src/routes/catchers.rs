use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

use super::errors::ApiError;

#[catch(404)]
fn page_not_found() -> Template {
    Template::render("404", context! {})
}

#[catch(404)]
fn api_route_not_found() -> ApiError {
    ApiError::custom(Status::NotFound, Some(String::from("Not found")))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CATCHERS", |rocket| async {
        rocket
            .register("/", catchers![page_not_found])
            .register("/api", catchers![api_route_not_found])
    })
}
