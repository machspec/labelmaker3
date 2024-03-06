use std::collections::HashMap;

mod actions;
mod config;

pub use config::Config;

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = Vec::new();
    routes.extend(actions::routes());
    routes
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Data {
    String(String),
    Number(i32),
    Boolean(bool),
    Object(HashMap<String, Self>),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub app_title: String,
    pub app_version: &'static str,
    pub data: HashMap<String, Data>,
}

impl State {
    pub fn new(app_title: &str) -> Self {
        let data = HashMap::new();

        Self {
            app_title: app_title.to_owned(),
            app_version: clap::crate_version!(),
            data,
        }
    }
}
