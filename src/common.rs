use std::{fs, io};

pub async fn get_page(file_name: &str) -> io::Result<String> {
    fs::read_to_string(format!("static/{file_name}"))
}