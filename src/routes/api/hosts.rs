use rocket::http::Status;

use crate::{configuration::CONFIGURATION, routes::errors::ApiError, wol::Wake};

#[post("/hosts/<name>")]
fn wake_up_host(name: &str) -> Result<Status, ApiError> {
    let config = match CONFIGURATION.read() {
        Ok(config) => Some(config),
        Err(error) => {
            println!("[API] Error while reading configuration {}", error);
            None
        }
    }.ok_or_else(|| ApiError::internal_error())?;

    if let Some(hosts) = &config.hosts {
        for (host_name, host) in hosts {
            if host_name.as_str() == name {
                host.wake();
                return Ok(Status::Ok)
            } else {
                return Err(ApiError::not_found(None))
            }
        }
    }

    Err(ApiError::not_found(None))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("HOSTS", |rocket| async {
        rocket
        .mount("/api", routes![wake_up_host])
    })
}