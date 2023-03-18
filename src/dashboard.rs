use rocket::response::content::RawHtml;
use crate::common::*;

#[get("/dashboard")]
pub async fn dashboard_serve() -> RawHtml<String> {
    RawHtml(
        get_page("dashboard.html")
            .await
            .expect("Could not load dashboard file!"),
    )
}