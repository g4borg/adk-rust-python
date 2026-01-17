//! Session management wrappers for Python

use pyo3::prelude::*;
use std::sync::Arc;
use std::collections::HashMap;

/// In-memory session service
#[pyclass(name = "InMemorySessionService")]
#[derive(Clone)]
pub struct PyInMemorySessionService {
    pub(crate) inner: Arc<adk_session::InMemorySessionService>,
}

#[pymethods]
impl PyInMemorySessionService {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(adk_session::InMemorySessionService::new()),
        }
    }

    fn __repr__(&self) -> String {
        "InMemorySessionService()".to_string()
    }
}

/// Session state wrapper
#[pyclass(name = "State")]
#[derive(Clone)]
pub struct PyState {
    data: HashMap<String, serde_json::Value>,
}

#[pymethods]
impl PyState {
    #[new]
    fn new() -> Self {
        Self { data: HashMap::new() }
    }

    fn get(&self, py: Python<'_>, key: &str) -> PyObject {
        match self.data.get(key) {
            Some(value) => pythonize::pythonize(py, value)
                .map(|b| b.into())
                .unwrap_or_else(|_| py.None()),
            None => py.None(),
        }
    }

    fn set(&mut self, key: String, value: &Bound<'_, PyAny>) -> PyResult<()> {
        let json_value: serde_json::Value = pythonize::depythonize(value)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        self.data.insert(key, json_value);
        Ok(())
    }

    fn all(&self, py: Python<'_>) -> PyObject {
        pythonize::pythonize(py, &self.data)
            .map(|b| b.into())
            .unwrap_or_else(|_| py.None())
    }

    fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn remove(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    fn __repr__(&self) -> String {
        format!("State(keys={})", self.data.len())
    }
}

/// Request to create a new session
#[pyclass(name = "CreateSessionRequest")]
#[derive(Clone)]
pub struct PyCreateSessionRequest {
    pub(crate) app_name: String,
    pub(crate) user_id: String,
    pub(crate) session_id: Option<String>,
    pub(crate) state: HashMap<String, serde_json::Value>,
}

#[pymethods]
impl PyCreateSessionRequest {
    #[new]
    #[pyo3(signature = (app_name, user_id, session_id=None))]
    fn new(app_name: String, user_id: String, session_id: Option<String>) -> Self {
        Self {
            app_name,
            user_id,
            session_id,
            state: HashMap::new(),
        }
    }

    fn with_state<'a>(mut slf: PyRefMut<'a, Self>, key: String, value: &Bound<'a, PyAny>) -> PyResult<PyRefMut<'a, Self>> {
        let json_value: serde_json::Value = pythonize::depythonize(value)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        slf.state.insert(key, json_value);
        Ok(slf)
    }
}

/// Request to get an existing session
#[pyclass(name = "GetSessionRequest")]
#[derive(Clone)]
pub struct PyGetSessionRequest {
    pub(crate) app_name: String,
    pub(crate) user_id: String,
    pub(crate) session_id: String,
}

#[pymethods]
impl PyGetSessionRequest {
    #[new]
    fn new(app_name: String, user_id: String, session_id: String) -> Self {
        Self { app_name, user_id, session_id }
    }
}

/// Streaming mode for agent execution
#[pyclass(name = "StreamingMode", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PyStreamingMode {
    None = 0,
    SSE = 1,
    Bidi = 2,
}

/// Run configuration for agent execution
#[pyclass(name = "RunConfig")]
#[derive(Clone)]
pub struct PyRunConfig {
    pub(crate) streaming_mode: PyStreamingMode,
}

#[pymethods]
impl PyRunConfig {
    #[new]
    #[pyo3(signature = (streaming_mode=PyStreamingMode::SSE))]
    fn new(streaming_mode: PyStreamingMode) -> Self {
        Self { streaming_mode }
    }

    #[getter]
    fn streaming_mode(&self) -> PyStreamingMode {
        self.streaming_mode
    }

    fn __repr__(&self) -> String {
        format!("RunConfig(streaming_mode={:?})", self.streaming_mode)
    }
}

impl From<PyRunConfig> for adk_core::RunConfig {
    fn from(config: PyRunConfig) -> Self {
        let mode = match config.streaming_mode {
            PyStreamingMode::None => adk_core::StreamingMode::None,
            PyStreamingMode::SSE => adk_core::StreamingMode::SSE,
            PyStreamingMode::Bidi => adk_core::StreamingMode::Bidi,
        };
        adk_core::RunConfig { streaming_mode: mode }
    }
}
