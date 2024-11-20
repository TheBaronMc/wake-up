use rocket::http::Status;

use crate::{
    configuration::read_configuration,
    routes::{errors::ApiError, guard::token::Token},
    wol::Wake,
};

#[post("/<groupname>")]
fn wake_up_group(
    authorization: Result<Token, ApiError>,
    groupname: &str,
) -> Result<Status, ApiError> {
    authorization?;

    let config = read_configuration().ok_or_else(|| {
        error!("[Groups] No configuration found");
        ApiError::internal()
    })?;

    let group = config
        .groups()
        .as_ref()
        .and_then(|groups| groups.get(groupname))
        .ok_or_else(|| {
            let message: String = format!("No group {} found", groupname).to_string();
            info!("[Groups] {}", message);
            ApiError::not_found(Some(message))
        })?;

    group.wake();
    Ok(Status::Ok)
}

#[post("/<groupname>/<hostname>")]
fn wake_up_group_host(
    authorization: Result<Token, ApiError>,
    groupname: &str,
    hostname: &str,
) -> Result<Status, ApiError> {
    authorization?;

    let config = read_configuration().ok_or_else(|| {
        error!("[Groups] No configuration found");
        ApiError::internal()
    })?;

    let host = config
        .groups()
        .as_ref()
        .and_then(|groups| groups.get(groupname))
        .and_then(|group| group.hosts.get(hostname))
        .ok_or_else(|| {
            let message: String =
                format!("No host {} found in group {}", hostname, groupname).to_string();
            info!("[Groups] {}", message);
            ApiError::not_found(Some(message))
        })?;

    host.wake();
    Ok(Status::Ok)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("GROUPS", |rocket| async {
        rocket.mount("/api/groups", routes![wake_up_group, wake_up_group_host])
    })
}
