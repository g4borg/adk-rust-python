//! Session management bindings for Python
//!
//! This module provides Python bindings for session management:
//! - `InMemorySessionService` - In-memory session storage
//! - `State` - Session state (key-value store)
//! - `RunConfig` - Agent execution configuration
//! - `StreamingMode` - Streaming behavior enum
//! - `CreateSessionRequest` - Request to create a session
//! - `GetSessionRequest` - Request to retrieve a session

use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

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
        Self {
            data: HashMap::new(),
        }
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

impl PyState {
    /// Create an empty state
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Create from adk_core::State
    pub fn from_session_state(state: &dyn adk_core::State) -> Self {
        Self { data: state.all() }
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

    fn with_state<'a>(
        mut slf: PyRefMut<'a, Self>,
        key: String,
        value: &Bound<'a, PyAny>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let json_value: serde_json::Value = pythonize::depythonize(value)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        slf.state.insert(key, json_value);
        Ok(slf)
    }

    fn __repr__(&self) -> String {
        format!(
            "CreateSessionRequest(app='{}', user='{}')",
            self.app_name, self.user_id
        )
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
        Self {
            app_name,
            user_id,
            session_id,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "GetSessionRequest(app='{}', user='{}', session='{}')",
            self.app_name, self.user_id, self.session_id
        )
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
        adk_core::RunConfig {
            streaming_mode: mode,
        }
    }
}

/// Model generation configuration
///
/// Controls LLM generation parameters like temperature, top_p, etc.
#[pyclass(name = "GenerateContentConfig")]
#[derive(Clone, Default)]
pub struct PyGenerateContentConfig {
    #[pyo3(get, set)]
    pub temperature: Option<f32>,
    #[pyo3(get, set)]
    pub top_p: Option<f32>,
    #[pyo3(get, set)]
    pub top_k: Option<i32>,
    #[pyo3(get, set)]
    pub max_output_tokens: Option<i32>,
    response_schema: Option<serde_json::Value>,
}

#[pymethods]
impl PyGenerateContentConfig {
    #[new]
    #[pyo3(signature = (temperature=None, top_p=None, top_k=None, max_output_tokens=None, response_schema=None))]
    fn new(
        temperature: Option<f32>,
        top_p: Option<f32>,
        top_k: Option<i32>,
        max_output_tokens: Option<i32>,
        response_schema: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<Self> {
        let schema = if let Some(dict) = response_schema {
            Some(
                pythonize::depythonize::<serde_json::Value>(dict.as_any())
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            )
        } else {
            None
        };

        Ok(Self {
            temperature,
            top_p,
            top_k,
            max_output_tokens,
            response_schema: schema,
        })
    }

    /// Get response schema as a Python dict
    #[getter]
    fn response_schema(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        match &self.response_schema {
            Some(schema) => pythonize::pythonize(py, schema)
                .map(|b| b.into())
                .unwrap_or_else(|_| py.None()),
            None => py.None(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "GenerateContentConfig(temperature={:?}, top_p={:?}, top_k={:?}, max_output_tokens={:?})",
            self.temperature, self.top_p, self.top_k, self.max_output_tokens
        )
    }
}

impl From<PyGenerateContentConfig> for adk_core::GenerateContentConfig {
    fn from(config: PyGenerateContentConfig) -> Self {
        Self {
            temperature: config.temperature,
            top_p: config.top_p,
            top_k: config.top_k,
            max_output_tokens: config.max_output_tokens,
            response_schema: config.response_schema,
        }
    }
}
