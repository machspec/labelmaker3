#[macro_use]
extern crate rocket;

use base64::Engine;
use rand::Rng;
use rocket::fs::{relative, FileServer};
use std::sync::{Arc, Mutex};

mod app;
mod constants;

use constants::{ADDRESS, PORT, SECRET_KEY, SECRET_KEY_LENGTH};

pub type AppState = Arc<Mutex<app::State>>;

// The title of the application. Adjust as needed.
pub const APP_TITLE: &str = "LabelMaker";

fn generate_secret_key() -> String {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..SECRET_KEY_LENGTH).map(|_| rng.gen()).collect();
    base64::engine::general_purpose::STANDARD.encode(key)
}

#[rocket::launch]
async fn rocket() -> _ {
    let config = app::Config::load();
    let state = app::State::new(APP_TITLE);
    let figment = rocket::Config::figment()
        .merge((ADDRESS, config.host()))
        .merge((PORT, config.port()))
        .merge((SECRET_KEY, generate_secret_key()));

    rocket::custom(figment)
        .manage(Arc::new(Mutex::new(state)))
        .mount("/", FileServer::from(relative!["../frontend/dist"]))
        .mount("/api", app::routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD;
    use rocket::local::asynchronous::Client;

    const ROCKET_EXPECT_MESSAGE: &str = "Valid rocket instance";

    #[test]
    fn test_generate_secret_key_length() {
        let key = generate_secret_key();
        let sample_bytes: Vec<u8> = (0..SECRET_KEY_LENGTH).map(|_| 0).collect();
        let expected_length = STANDARD.encode(sample_bytes).len();
        assert_eq!(key.len(), expected_length);
    }

    #[test]
    fn test_generate_secret_key_format() {
        let key = generate_secret_key();
        assert!(STANDARD.decode(key).is_ok());
    }

    #[rocket::async_test]
    async fn test_valid_server_request() {
        let rocket = rocket().await;
        let client = Client::tracked(rocket).await.expect(ROCKET_EXPECT_MESSAGE);
        let response = client.get("/").dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[rocket::async_test]
    async fn test_invalid_server_request() {
        let rocket = rocket().await;
        let client = Client::tracked(rocket).await.expect(ROCKET_EXPECT_MESSAGE);
        let response = client.get("/invalid").dispatch().await;
        assert_eq!(response.status(), rocket::http::Status::NotFound);
    }
}