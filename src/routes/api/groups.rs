use rocket::http::Status;

use crate::{configuration::read_configuration, routes::errors::ApiError, wol::Wake};

#[post("/groups/<groupname>")]
fn wake_up_group(groupname: &str) -> Result<Status, ApiError> {
    let config = read_configuration().ok_or_else(|| ApiError::not_found(None))?;

    let group = config
        .groups()
        .as_ref()
        .and_then(|groups| groups.get(groupname))
        .ok_or_else(|| {
            ApiError::not_found(Some(String::from(format!("No group {} found", groupname))))
        })?;

    group.wake();
    Ok(Status::Ok)
}

#[post("/groups/<groupname>/<hostname>")]
fn wake_up_group_host(groupname: &str, hostname: &str) -> Result<Status, ApiError> {
    let config = read_configuration().ok_or_else(|| ApiError::not_found(None))?;

    let host = config
        .groups()
        .as_ref()
        .and_then(|groups| groups.get(groupname))
        .and_then(|group| group.hosts.get(hostname))
        .ok_or_else(|| {
            ApiError::not_found(Some(String::from(format!(
                "No host {} found in group {}",
                hostname, groupname
            ))))
        })?;

    host.wake();
    Ok(Status::Ok)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("GROUPS", |rocket| async {
        rocket.mount("/api", routes![wake_up_group, wake_up_group_host])
    })
}
