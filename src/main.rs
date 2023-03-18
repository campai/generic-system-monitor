mod observer;
mod dashboard;
mod common;

#[macro_use]
extern crate rocket;

use rocket::routes;
use observer::*;
use dashboard::dashboard_serve;

const VERSION: &str = "v1";

#[launch]
async fn start() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount(
        format!("/gsm/api/{VERSION}"),
        routes![dashboard_serve, observers, run],
    )
}
