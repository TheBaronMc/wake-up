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

#[catch(default)]
fn api_internal_error() -> ApiError {
    ApiError::internal()
}

pub fn stage(web_enable: bool, api_enable: bool) -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("CATCHERS", move |mut rocket| async move {
        if web_enable {
            rocket = rocket.register("/", catchers![page_not_found])
        } else {
            rocket = rocket.register("/", catchers![api_route_not_found])
        }

        if api_enable {
            rocket = rocket.register("/api", catchers![api_route_not_found, api_internal_error])
        }

        rocket
    })
}
