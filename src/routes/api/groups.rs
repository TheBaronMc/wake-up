use rocket::http::Status;

use crate::{configuration::CONFIGURATION, routes::errors::ApiError, wol::Wake};

#[post("/groups/<groupname>")]
fn wake_up_group(groupname: &str) -> Result<Status, ApiError> {
    let config = match CONFIGURATION.read() {
        Ok(config) => Some(config),
        Err(error) => {
            println!("[API] Error while reading configuration {}", error);
            None
        }
    }.ok_or_else(|| ApiError::internal_error())?;

    if let Some(groups) = &config.groups {
        for (group_name, group) in groups {
            if group_name.as_str() == groupname {
                group.wake();
                return Ok(Status::Ok)
            } else {
                return Err(ApiError::not_found(None))
            }
        }
    }

    Err(ApiError::not_found(None))
}

#[post("/groups/<groupname>/<hostname>")]
fn wake_up_group_host(groupname: &str, hostname: &str) -> Result<Status, ApiError> {
    let config = match CONFIGURATION.read() {
        Ok(config) => Some(config),
        Err(error) => {
            println!("[API] Error while reading configuration {}", error);
            None
        }
    }.ok_or_else(|| ApiError::internal_error())?;

    if let Some(groups) = &config.groups {
        match groups.get(groupname) {
            Some(group) => {
                match group.hosts.get(hostname) {
                    Some(host) => {
                        host.wake();
                        return Ok(Status::Ok)
                    }
                    None => return Err(ApiError::not_found(None))
                }
            }
            None => return Err(ApiError::not_found(None))
        }
    }

    Err(ApiError::not_found(None))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("GROUPS", |rocket| async {
        rocket
        .mount("/api", routes![wake_up_group, wake_up_group_host])
    })
}