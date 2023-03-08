#[macro_use]
extern crate rocket;

use std::io::Read;

use rocket::{get, routes};
use rocket::form::Form;
use rocket::response::content::RawHtml;

const VERSION: &str = "v1";

#[get("/info")]
fn info() -> RawHtml<String> {
    let content = get_page("observers.html", "{{RUN END-POINT}}", &format!("/api/{VERSION}/observer/run"));
    RawHtml(content)
}

fn get_page(page_path: &str, ep_placeholder: &str, ep_path: &str) -> String {
    let mut page = String::new();
    std::fs::File::open("static/".to_owned() + page_path)
        .expect("Could not open observers HTML file!")
        .read_to_string(&mut page)
        .expect("Could not read from observers file!");

    page.replace(ep_placeholder, ep_path)
}

#[derive(FromForm)]
struct RunRequest<'life> {
    name: &'life str,
    delay: u32,
}

#[post("/observer/run", format = "application/x-www-form-urlencoded", data = "<form>")]
fn run(form: Form<RunRequest<'_>>) -> String {
    format!("Received: {}, {}", form.name, form.delay)
}

#[launch]
fn start() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(format!("/api/{VERSION}"), routes![info, run])
}
