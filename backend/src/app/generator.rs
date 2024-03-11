use crate::constants::generator::*;
use pyo3::prelude::*;

pub fn generate_label_file(data: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();

    let generator = include_str!("generator.py");
    let result = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, &generator, "", "")?
            .getattr(GENERATOR_ENTRYPOINT)?
            .into();

        app.call1(py, (data,))
    });

    Ok(format!("{}", result?))
}
