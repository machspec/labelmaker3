use super::generator::generate_label_file;
use crate::AppState;
use rocket::{serde::json::Json, State};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_state, print_label]
}

#[get("/state")]
pub async fn get_state(state: &State<AppState>) -> Json<AppState> {
    let state = state.inner().clone();
    Json(state)
}

#[post("/print_label", data = "<data>")]
pub async fn print_label(data: &str) -> String {
    match generate_label_file(data) {
        Ok(py_app) => format!("py: {}", py_app),
        Err(e) => format!("Error: {}", e),
    }
}
