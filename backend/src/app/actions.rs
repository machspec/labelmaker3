use super::generator::generate_label_file;
use crate::AppState;
use rocket::{serde::json::Json, State};

use std::env;
use std::fs::File;
use std::path::Path;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_state, print_label]
}

#[get("/state")]
pub async fn get_state(state: &State<AppState>) -> Json<AppState> {
    let state = state.inner().clone();
    Json(state)
}

#[post("/print_label", data = "<data>")]
pub fn print_label(data: &str) -> Result<Option<File>, String> {
    match generate_label_file(data) {
        Ok(_) => {
            // Prepare path string to upload folder
            let pdf_dir = Path::new("pdf").join("labels.pdf");

            // Get the current working directory
            let cwd = env::current_dir().expect("Failed to get current working directory");

            // Join the CWD and pdf_dir
            let file_path = cwd.join(pdf_dir);

            // Load file
            Ok(File::open(file_path).ok())
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}
