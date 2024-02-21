use crate::AppState;
use rocket::{serde::json::Json, State};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_state]
}

#[get("/state")]
pub async fn get_state(state: &State<AppState>) -> Json<AppState> {
    let state = state.inner().clone();
    Json(state)
}
