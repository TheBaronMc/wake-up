use std::path::{Path, PathBuf};

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use rocket::http::CookieJar;

use crate::{
    auth::{create_token, verify_pass},
    configuration::{read_global_configuration, Configuration},
};

static SESSION_TOKEN_KEY: &str = "token";

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    match cookies.get(SESSION_TOKEN_KEY) {
        Some(_) => Ok(read_global_configuration(|global_configuration| {
            let configuration: &Configuration = global_configuration.unwrap();
            Template::render(
                "index",
                context! {
                    groups: configuration.groups(),
                    hosts: configuration.hosts()
                },
            )
        })),
        None => Err(Redirect::to(uri!(login_get))),
    }
}

#[get("/login")]
fn login_get(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    match cookies.get(SESSION_TOKEN_KEY) {
        Some(_) => Err(Redirect::to(uri!(index))),
        None => Ok(Template::render("login", context! {})),
    }
}

#[derive(FromForm)]
struct Credentials<'r> {
    password: &'r str,
}

#[post("/login", data = "<credentials>")]
fn login_post(
    cookies: &CookieJar<'_>,
    credentials: Form<Credentials<'_>>,
) -> Result<Redirect, Redirect> {
    match cookies.get("token") {
        Some(_) => Err(Redirect::to(uri!(index))),
        None => {
            if verify_pass(credentials.password) {
                let token = create_token().expect("Unexpected error");
                cookies.add((SESSION_TOKEN_KEY, token));
                Ok(Redirect::to(uri!(index)))
            } else {
                Err(Redirect::to(uri!(login_get)))
            }
        }
    }
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(file)).await.ok()
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("PAGES", |rocket| async {
        rocket
            .mount("/", routes![index, login_get, login_post, files])
            .attach(Template::fairing())
    })
}
