//! CustomAgent bindings (stub - to be implemented)

use pyo3::prelude::*;

/// A custom agent with user-defined logic (not using an LLM)
///
/// NOTE: This is currently a stub. Full implementation will allow
/// users to define custom async run logic in Python.
#[pyclass(name = "CustomAgent")]
pub struct PyCustomAgent {
    name: String,
    description: String,
}

#[pymethods]
impl PyCustomAgent {
    #[new]
    fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    fn description(&self) -> String {
        self.description.clone()
    }

    fn __repr__(&self) -> String {
        format!("CustomAgent(name='{}')", self.name)
    }
}
