use rocket::http::Status;

use crate::{
    configuration::{read_configuration, Configuration},
    routes::errors::ApiError,
    wol::Wake,
};

#[post("/hosts/<name>")]
fn wake_up_host(name: &str) -> Result<Status, ApiError> {
    let config: Configuration = read_configuration()
        .ok_or_else(|| ApiError::not_found(Some(format!("No user named {}", name))))?;

    let host = config
        .hosts()
        .as_ref()
        .and_then(|hosts| hosts.get(name))
        .ok_or_else(|| {
            ApiError::not_found(Some(String::from(format!("No host {} found", name))))
        })?;

    host.wake();
    Ok(Status::Ok)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("HOSTS", |rocket| async {
        rocket.mount("/api", routes![wake_up_host])
    })
}
