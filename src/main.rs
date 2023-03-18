#[macro_use]
extern crate rocket;

use std::path::Path;

use rocket::fs::NamedFile;
use rocket::routes;

use dashboard::dashboard_serve;
use observer::*;

mod observer;
mod dashboard;
mod common;

const VERSION: &str = "v1";

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[launch]
async fn start() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/", routes![favicon],
        )
        .mount(
            format!("/gsm/api/{VERSION}"),
            routes![favicon, dashboard_serve, observers, run],
        )
}
