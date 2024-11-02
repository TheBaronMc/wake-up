use rocket::http::Status;

use crate::{configuration::CONFIGURATION, wol::Wake};

#[post("/hosts/<name>")]
fn wake_up_host(name: &str) -> Result<Status, Status> {
    let config = CONFIGURATION.read().expect("Unexpected error");

    if let Some(hosts) = &config.hosts {
        for (host_name, host) in hosts {
            if host_name.as_str() == name {
                host.wake();
                return Ok(Status::Ok)
            } else {
                return Err(Status::NotFound)
            }
        }
    }

    Err(Status::NotFound)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("HOSTS", |rocket| async {
        rocket
        .mount("/api", routes![wake_up_host])
    })
}