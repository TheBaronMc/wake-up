use rocket::http::Status;

use crate::{configuration::CONFIGURATION, wol::Wake};

#[post("/groups/<groupname>")]
fn wake_up_group(groupname: &str) -> Result<Status, Status> {
    let config = CONFIGURATION.read().expect("Unexpected error");

    if let Some(groups) = &config.groups {
        for (group_name, group) in groups {
            if group_name.as_str() == groupname {
                group.wake();
                return Ok(Status::Ok)
            } else {
                return Err(Status::NotFound)
            }
        }
    }

    Err(Status::NotFound)
}

#[post("/groups/<groupname>/<hostname>")]
fn wake_up_group_host(groupname: &str, hostname: &str) -> Result<Status, Status> {
    let config = CONFIGURATION.read().expect("Unexpected error");

    if let Some(groups) = &config.groups {
        match groups.get(groupname) {
            Some(group) => {
                match group.hosts.get(hostname) {
                    Some(host) => {
                        host.wake();
                        return Ok(Status::Ok)
                    }
                    None => return Err(Status::NotFound)
                }
            }
            None => return Err(Status::NotFound)
        }
    }

    Err(Status::NotFound)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("GROUPS", |rocket| async {
        rocket
        .mount("/api", routes![wake_up_group, wake_up_group_host])
    })
}