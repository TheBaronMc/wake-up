#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, http::Status};
use rocket_dyn_templates::{context, Template};

mod host;
mod group;
mod configuration;
mod wol;

use configuration::{read_configuration, CONFIGURATION};

static CONFIGURATION_PATH: &str = "configuration.yml";

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

#[get("/reload")]
fn reaload() -> Status {
    match read_configuration(CONFIGURATION_PATH) {
        Ok(_) => Status::Ok,
        Err(_error) => Status::NotModified
    }
}

#[rocket::main]
async fn main() -> () {
    if let Err(error) = read_configuration(CONFIGURATION_PATH) {
        println!("{}", error);
        return
    }

    let _rocket = rocket::build()
        .mount("/", routes![index, login, files, reaload])
        .attach(Template::fairing())
        .launch()
        .await
        .expect("Error while lauching rocket");
    
    ()
}