//! Session management bindings for Python
//!
//! This module provides Python bindings for session management:
//! - `InMemorySessionService` - In-memory session storage with full CRUD
//! - `Session` - Session wrapper with access to id, state, events
//! - `State` - Session state (key-value store)
//! - `RunConfig` - Agent execution configuration
//! - `StreamingMode` - Streaming behavior enum
//! - `CreateSessionRequest` - Request to create a session
//! - `GetSessionRequest` - Request to retrieve a session
//! - `ListSessionRequest` - Request to list sessions
//! - `DeleteSessionRequest` - Request to delete a session

use adk_session::SessionService;
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::PyEvent;

/// Session wrapper providing access to session data
#[pyclass(name = "Session")]
#[derive(Clone)]
pub struct PySession {
    id: String,
    app_name: String,
    user_id: String,
    state: PyState,
    events: Vec<PyEvent>,
    last_update_time: DateTime<Utc>,
}

#[pymethods]
impl PySession {
    /// Get the session ID
    #[getter]
    fn id(&self) -> &str {
        &self.id
    }

    /// Get the application name
    #[getter]
    fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Get the user ID
    #[getter]
    fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Get the session state
    #[getter]
    fn state(&self) -> PyState {
        self.state.clone()
    }

    /// Get all events in the session
    #[getter]
    fn events(&self) -> Vec<PyEvent> {
        self.events.clone()
    }

    /// Get the last update timestamp as ISO 8601 string
    #[getter]
    fn last_update_time(&self) -> String {
        self.last_update_time.to_rfc3339()
    }

    /// Get the number of events in the session
    fn event_count(&self) -> usize {
        self.events.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "Session(id='{}', app='{}', user='{}', events={})",
            self.id,
            self.app_name,
            self.user_id,
            self.events.len()
        )
    }
}

impl PySession {
    /// Create from a Rust Session trait object
    pub fn from_session(session: &dyn adk_session::Session) -> Self {
        let events = session
            .events()
            .all()
            .into_iter()
            .map(PyEvent::from)
            .collect();

        Self {
            id: session.id().to_string(),
            app_name: session.app_name().to_string(),
            user_id: session.user_id().to_string(),
            state: PyState::from_session_state(session.state()),
            events,
            last_update_time: session.last_update_time(),
        }
    }
}

/// In-memory session service with full CRUD operations
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

    /// Create a new session
    ///
    /// Args:
    ///     request: CreateSessionRequest with app_name, user_id, optional session_id
    ///
    /// Returns:
    ///     Session: The created session
    fn create<'py>(
        &self,
        py: Python<'py>,
        request: &PyCreateSessionRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let service = self.inner.clone();
        let req = adk_session::CreateRequest {
            app_name: request.app_name.clone(),
            user_id: request.user_id.clone(),
            session_id: request.session_id.clone(),
            state: request.state.clone(),
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let session = service
                .create(req)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Ok(PySession::from_session(session.as_ref()))
        })
    }

    /// Get an existing session
    ///
    /// Args:
    ///     request: GetSessionRequest with app_name, user_id, session_id
    ///
    /// Returns:
    ///     Session: The retrieved session
    ///
    /// Raises:
    ///     RuntimeError: If session not found
    fn get<'py>(
        &self,
        py: Python<'py>,
        request: &PyGetSessionRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let service = self.inner.clone();
        let req = adk_session::GetRequest {
            app_name: request.app_name.clone(),
            user_id: request.user_id.clone(),
            session_id: request.session_id.clone(),
            num_recent_events: request.num_recent_events,
            after: request.after,
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let session = service
                .get(req)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Ok(PySession::from_session(session.as_ref()))
        })
    }

    /// List all sessions for a user
    ///
    /// Args:
    ///     request: ListSessionRequest with app_name, user_id
    ///
    /// Returns:
    ///     List[Session]: All sessions for the user
    fn list<'py>(
        &self,
        py: Python<'py>,
        request: &PyListSessionRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let service = self.inner.clone();
        let req = adk_session::ListRequest {
            app_name: request.app_name.clone(),
            user_id: request.user_id.clone(),
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let sessions = service
                .list(req)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let py_sessions: Vec<PySession> = sessions
                .iter()
                .map(|s| PySession::from_session(s.as_ref()))
                .collect();
            Ok(py_sessions)
        })
    }

    /// Delete a session
    ///
    /// Args:
    ///     request: DeleteSessionRequest with app_name, user_id, session_id
    ///
    /// Raises:
    ///     RuntimeError: If session not found
    fn delete<'py>(
        &self,
        py: Python<'py>,
        request: &PyDeleteSessionRequest,
    ) -> PyResult<Bound<'py, PyAny>> {
        let service = self.inner.clone();
        let req = adk_session::DeleteRequest {
            app_name: request.app_name.clone(),
            user_id: request.user_id.clone(),
            session_id: request.session_id.clone(),
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            service
                .delete(req)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            Ok(())
        })
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

    fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    fn __len__(&self) -> usize {
        self.data.len()
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
    pub fn from_core_state(state: &dyn adk_core::State) -> Self {
        Self { data: state.all() }
    }

    /// Create from adk_session::State
    pub fn from_session_state(state: &dyn adk_session::State) -> Self {
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

    #[getter]
    fn app_name(&self) -> &str {
        &self.app_name
    }

    #[getter]
    fn user_id(&self) -> &str {
        &self.user_id
    }

    #[getter]
    fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
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
            "CreateSessionRequest(app='{}', user='{}', session={:?})",
            self.app_name, self.user_id, self.session_id
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
    pub(crate) num_recent_events: Option<usize>,
    pub(crate) after: Option<DateTime<Utc>>,
}

#[pymethods]
impl PyGetSessionRequest {
    #[new]
    #[pyo3(signature = (app_name, user_id, session_id, num_recent_events=None))]
    fn new(
        app_name: String,
        user_id: String,
        session_id: String,
        num_recent_events: Option<usize>,
    ) -> Self {
        Self {
            app_name,
            user_id,
            session_id,
            num_recent_events,
            after: None,
        }
    }

    #[getter]
    fn app_name(&self) -> &str {
        &self.app_name
    }

    #[getter]
    fn user_id(&self) -> &str {
        &self.user_id
    }

    #[getter]
    fn session_id(&self) -> &str {
        &self.session_id
    }

    #[getter]
    fn num_recent_events(&self) -> Option<usize> {
        self.num_recent_events
    }

    fn __repr__(&self) -> String {
        format!(
            "GetSessionRequest(app='{}', user='{}', session='{}')",
            self.app_name, self.user_id, self.session_id
        )
    }
}

/// Request to list sessions for a user
#[pyclass(name = "ListSessionRequest")]
#[derive(Clone)]
pub struct PyListSessionRequest {
    pub(crate) app_name: String,
    pub(crate) user_id: String,
}

#[pymethods]
impl PyListSessionRequest {
    #[new]
    fn new(app_name: String, user_id: String) -> Self {
        Self { app_name, user_id }
    }

    #[getter]
    fn app_name(&self) -> &str {
        &self.app_name
    }

    #[getter]
    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn __repr__(&self) -> String {
        format!(
            "ListSessionRequest(app='{}', user='{}')",
            self.app_name, self.user_id
        )
    }
}

/// Request to delete a session
#[pyclass(name = "DeleteSessionRequest")]
#[derive(Clone)]
pub struct PyDeleteSessionRequest {
    pub(crate) app_name: String,
    pub(crate) user_id: String,
    pub(crate) session_id: String,
}

#[pymethods]
impl PyDeleteSessionRequest {
    #[new]
    fn new(app_name: String, user_id: String, session_id: String) -> Self {
        Self {
            app_name,
            user_id,
            session_id,
        }
    }

    #[getter]
    fn app_name(&self) -> &str {
        &self.app_name
    }

    #[getter]
    fn user_id(&self) -> &str {
        &self.user_id
    }

    #[getter]
    fn session_id(&self) -> &str {
        &self.session_id
    }

    fn __repr__(&self) -> String {
        format!(
            "DeleteSessionRequest(app='{}', user='{}', session='{}')",
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
