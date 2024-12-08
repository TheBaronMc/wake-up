use log::info;
use rocket::http::Status;

use crate::{
    configuration::read_global_configuration,
    host::Host,
    routes::{errors::ApiError, guard::token::Token},
    wol::Wake,
};

#[post("/hosts/<name>")]
fn wake_up_host(authorization: Result<Token, ApiError>, name: &str) -> Result<Status, ApiError> {
    authorization?;

    let found_host: Option<Host> = read_global_configuration(|global_configuration| {
        global_configuration
            .and_then(|configuration| {
                configuration
                    .hosts()
                    .as_ref()
                    .and_then(|hosts| hosts.get(name))
            })
            .cloned()
    });

    match found_host {
        Some(host) => {
            host.wake();
            Ok(Status::Ok)
        }
        None => {
            let message: String = format!("No host {} found", name);
            info!("[hosts] {message}");
            Err(ApiError::not_found(Some(message)))
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("HOSTS", |rocket| async {
        rocket.mount("/api", routes![wake_up_host])
    })
}
