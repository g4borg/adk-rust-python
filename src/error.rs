//! Error types for Python bindings

use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdkPyError {
    #[error("ADK error: {0}")]
    Adk(#[from] adk_core::AdkError),

    #[error("Python error: {0}")]
    Python(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<AdkPyError> for PyErr {
    fn from(err: AdkPyError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

impl From<PyErr> for AdkPyError {
    fn from(err: PyErr) -> Self {
        AdkPyError::Python(err.to_string())
    }
}

/// Python-visible error class
#[pyclass(name = "AdkError")]
#[derive(Clone)]
pub struct PyAdkError {
    message: String,
}

#[pymethods]
impl PyAdkError {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }

    fn __str__(&self) -> String {
        self.message.clone()
    }

    fn __repr__(&self) -> String {
        format!("AdkError('{}')", self.message)
    }

    #[getter]
    fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<adk_core::AdkError> for PyAdkError {
    fn from(err: adk_core::AdkError) -> Self {
        Self { message: err.to_string() }
    }
}

pub type PyResult<T> = Result<T, AdkPyError>;
