use super::generator::generate_label_file;
use crate::AppState;
use rocket::{serde::json::Json, State};

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
            let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "pdf");

            // Get path of the file by file name
            let filename = Path::new(upload_dir).join("labels.pdf");

            // Load file
            Ok(File::open(&filename).ok())
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}
