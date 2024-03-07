use crate::constants::generator::*;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::{fs, path::Path};

pub fn generate_label_file(data: &str) -> PyResult<String> {
    pyo3::prepare_freethreaded_python();

    let directory = Path::new(DIRECTORY);
    let generator = fs::read_to_string(directory.join(ENTRYPOINT))?;

    let result = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let path: &PyList = py.import(SYS)?.getattr(PATH)?.downcast()?;
        path.insert(0, &directory)?;

        let app: Py<PyAny> = PyModule::from_code(py, &generator, "", "")?
            .getattr(RUN)?
            .into();

        app.call1(py, (data,))
    });

    Ok(format!("{}", result?))
}
