use crate::AppState;
use rocket::{serde::json::Json, State};

use pyo3::prelude::*;
use pyo3::types::PyList;
use std::fs;
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
pub async fn print_label(data: &str) -> String {
    match generate_label_file(data) {
        Ok(py_app) => format!("py: {}", py_app),
        Err(e) => format!("Error: {}", e),
    }
}

fn generate_label_file(data: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();

    let path = Path::new("python_app");
    let py_app = fs::read_to_string(path.join("app.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("run")?
            .into();

        let args = (data,);
        app.call1(py, args)
    });

    Ok(format!("{}", from_python?))
}
