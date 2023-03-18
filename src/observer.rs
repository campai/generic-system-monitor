use std::process::Command;
use std::str;

use rocket::form::Form;
use rocket::response::content::RawJson;

#[derive(FromForm)]
pub struct RunRequest {
    #[field(validate = len(1..50))]
    command: String,
    #[field(validate = range(500..))]
    delay: u32,
}

#[post("/observer/<id>/run", format = "application/x-www-form-urlencoded", data = "<form>")]
pub async fn run(id: u32, form: Form<RunRequest>) -> String {
    dbg!("Received for observer {}: {}, {}", id, &form.command, form.delay);

    let output = Command::new(&form.command)
        .output()
        .expect("Failed to execute command.");

    format!(
        "stdio:\n{}\nstderr:\n{}",
        str::from_utf8(&output.stdout).unwrap(),
        str::from_utf8(&output.stderr).unwrap()
    )
}

#[get("/observer")]
pub async fn observers() -> RawJson<&'static str> {
    RawJson(r#"{
            "observers": [
                {
                    "id": 1,
                    "check": "check.sh",
                    "fix": "fix.sh",
                    "auto-fix": true,
                    "state": "RUNNING",
                    "last-run": "2023-03-04T05:06:07Z",
                    "run-times": 123,
                    "auto-fix-times": 11,
                    "manual-fix-times": 2,
                    "created-at": "2023-01-02T03:04:05Z",
                    "modified-at": "2023-02-03T04:05:06Z"
                }
            ]
        }"#)
}