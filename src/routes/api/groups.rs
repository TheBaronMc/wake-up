use rocket::http::Status;

use crate::{
    configuration::read_global_configuration,
    group::Group,
    host::Host,
    routes::{errors::ApiError, guard::token::Token},
    wol::Wake,
};

#[post("/<groupname>")]
fn wake_up_group(
    authorization: Result<Token, ApiError>,
    groupname: &str,
) -> Result<Status, ApiError> {
    authorization?;

    let found_group: Option<Group> = read_global_configuration(|global_configuration| {
        global_configuration
            .and_then(|configuration| {
                configuration
                    .groups()
                    .as_ref()
                    .and_then(|groups| groups.get(groupname))
            })
            .cloned()
    });

    match found_group {
        Some(group) => {
            group.wake();
            Ok(Status::Ok)
        }
        None => {
            let message: String = format!("No group {} found", groupname).to_string();
            info!("[Groups] {}", message);
            Err(ApiError::not_found(Some(message)))
        }
    }
}

#[post("/<groupname>/<hostname>")]
fn wake_up_group_host(
    authorization: Result<Token, ApiError>,
    groupname: &str,
    hostname: &str,
) -> Result<Status, ApiError> {
    authorization?;

    let found_host: Option<Host> = read_global_configuration(|global_configuration| {
        global_configuration
            .and_then(|configuration| {
                configuration
                    .groups()
                    .as_ref()
                    .and_then(|groups| groups.get(groupname))
                    .and_then(|group| group.hosts.get(hostname))
            })
            .cloned()
    });

    match found_host {
        Some(host) => {
            host.wake();
            Ok(Status::Ok)
        }
        None => {
            let message: String =
                format!("No host {} found in group {}", hostname, groupname).to_string();
            info!("[Groups] {}", message);
            Err(ApiError::not_found(Some(message)))
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("GROUPS", |rocket| async {
        rocket.mount("/api/groups", routes![wake_up_group, wake_up_group_host])
    })
}
