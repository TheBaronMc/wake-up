use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket_dyn_templates::{context, Template};

use crate::configuration::CONFIGURATION;

#[get("/")]
fn index() -> Template {
    let current_configuration = CONFIGURATION.read().expect("Failed to read configuration");

    Template::render("index", context! {
        groups: &current_configuration.groups,
        hosts: &current_configuration.hosts
    })
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context! {})
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(file)).await.ok()
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("PAGES", |rocket| async {
        rocket
        .mount("/", routes![index, login, files])
        .attach(Template::fairing())
    })
}