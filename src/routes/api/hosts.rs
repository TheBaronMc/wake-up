use log::info;
use rocket::http::Status;

use crate::{
    configuration::{read_global_configuration, Configuration},
    routes::{errors::ApiError, guard::token::Token},
    wol::Wake,
};

#[post("/hosts/<name>")]
fn wake_up_host(authorization: Result<Token, ApiError>, name: &str) -> Result<Status, ApiError> {
    authorization?;

    read_global_configuration(|global_configuration| {
        let config: Configuration = global_configuration.ok_or_else(|| {
            let message: String = format!("No user named {name}");
            error!("[hosts] {message}");
            ApiError::internal()
        })?;

        let host = config
            .hosts()
            .as_ref()
            .and_then(|hosts| hosts.get(name))
            .ok_or_else(|| {
                let message: String = format!("No host {} found", name);
                info!("[hosts] {message}");
                ApiError::not_found(Some(message))
            })?;

        host.wake();
        Ok(Status::Ok)
    })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("HOSTS", |rocket| async {
        rocket.mount("/api", routes![wake_up_host])
    })
}
