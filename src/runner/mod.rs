//! Runner bindings for executing agents
//!
//! This module provides Python bindings for agent execution:
//! - `Runner` - Execute agents with full configuration
//! - `EventStream` - Async iterator for streaming events
//! - `run_agent()` - Convenience function for simple execution

use adk_session::SessionService;
use futures::StreamExt;
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::agent::PyLlmAgent;
use crate::session::{PyInMemorySessionService, PyRunConfig};
use crate::types::PyEvent;

/// Async iterator for streaming events from agent execution.
///
/// Use with `async for`:
/// ```python
/// async for event in runner.run_stream(user_id, session_id, message):
///     print(event.get_text())
/// ```
#[pyclass(name = "EventStream")]
pub struct PyEventStream {
    receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Result<PyEvent, String>>>>,
}

#[pymethods]
impl PyEventStream {
    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let receiver = self.receiver.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut rx = receiver.lock().await;
            match rx.recv().await {
                Some(Ok(event)) => Ok(Some(event)),
                Some(Err(e)) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
                None => Ok(None), // Stream exhausted - signals StopAsyncIteration
            }
        })
    }
}

/// Runner for executing agents
#[pyclass(name = "Runner")]
pub struct PyRunner {
    app_name: String,
    agent: Arc<dyn adk_core::Agent>,
    session_service: Arc<dyn adk_session::SessionService>,
    run_config: Option<adk_core::RunConfig>,
}

#[pymethods]
impl PyRunner {
    #[new]
    #[pyo3(signature = (app_name, agent, session_service, run_config=None))]
    fn new(
        app_name: String,
        agent: &PyLlmAgent,
        session_service: &PyInMemorySessionService,
        run_config: Option<&PyRunConfig>,
    ) -> Self {
        Self {
            app_name,
            agent: agent.inner.clone(),
            session_service: session_service.inner.clone(),
            run_config: run_config.map(|c| c.clone().into()),
        }
    }

    /// Run the agent with the given user message, returning all events
    fn run<'py>(
        &self,
        py: Python<'py>,
        user_id: String,
        session_id: String,
        message: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let agent = self.agent.clone();
        let session_service = self.session_service.clone();
        let app_name = self.app_name.clone();
        let run_config = self.run_config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let user_content = adk_core::Content::new("user").with_text(&message);

            let config = adk_runner::RunnerConfig {
                app_name,
                agent,
                session_service,
                artifact_service: None,
                memory_service: None,
                run_config,
            };

            let runner = adk_runner::Runner::new(config)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            let stream_result = runner.run(user_id, session_id, user_content).await;
            let mut stream = stream_result
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            let mut events = Vec::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => events.push(PyEvent::from(event)),
                    Err(e) => return Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
                }
            }

            Ok(events)
        })
    }

    /// Run the agent and return just the final response text
    fn run_simple<'py>(
        &self,
        py: Python<'py>,
        user_id: String,
        session_id: String,
        message: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let agent = self.agent.clone();
        let session_service = self.session_service.clone();
        let app_name = self.app_name.clone();
        let run_config = self.run_config.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let user_content = adk_core::Content::new("user").with_text(&message);

            let config = adk_runner::RunnerConfig {
                app_name,
                agent,
                session_service,
                artifact_service: None,
                memory_service: None,
                run_config,
            };

            let runner = adk_runner::Runner::new(config)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            let stream_result = runner.run(user_id, session_id, user_content).await;
            let mut stream = stream_result
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            let mut final_text = String::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => {
                        if event.is_final_response() {
                            if let Some(content) = event.content() {
                                for part in content.parts.iter() {
                                    if let Some(text) = part.text() {
                                        final_text.push_str(text);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => return Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
                }
            }

            Ok(final_text)
        })
    }

    /// Run the agent with streaming - returns an async iterator of events.
    ///
    /// Use with `async for`:
    /// ```python
    /// async for event in runner.run_stream(user_id, session_id, message):
    ///     if text := event.get_text():
    ///         print(text, end="", flush=True)
    /// ```
    fn run_stream(
        &self,
        _py: Python<'_>,
        user_id: String,
        session_id: String,
        message: String,
    ) -> PyResult<PyEventStream> {
        let agent = self.agent.clone();
        let session_service = self.session_service.clone();
        let app_name = self.app_name.clone();
        let run_config = self.run_config.clone();

        // Create a channel for sending events
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        // Spawn a task that reads from the Rust stream and sends to the channel
        pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
            let user_content = adk_core::Content::new("user").with_text(&message);

            let config = adk_runner::RunnerConfig {
                app_name,
                agent,
                session_service,
                artifact_service: None,
                memory_service: None,
                run_config,
            };

            let runner = match adk_runner::Runner::new(config) {
                Ok(r) => r,
                Err(e) => {
                    let _ = tx.send(Err(e.to_string())).await;
                    return;
                }
            };

            let stream_result = runner.run(user_id, session_id, user_content).await;
            let mut stream = match stream_result {
                Ok(s) => s,
                Err(e) => {
                    let _ = tx.send(Err(e.to_string())).await;
                    return;
                }
            };

            while let Some(result) = stream.next().await {
                let send_result = match result {
                    Ok(event) => tx.send(Ok(PyEvent::from(event))).await,
                    Err(e) => tx.send(Err(e.to_string())).await,
                };

                if send_result.is_err() {
                    // Receiver dropped, stop sending
                    break;
                }
            }
            // Channel closes when tx is dropped
        });

        Ok(PyEventStream {
            receiver: Arc::new(Mutex::new(rx)),
        })
    }

    fn __repr__(&self) -> String {
        format!("Runner(app_name='{}')", self.app_name)
    }
}

/// Simple function to run an agent once
#[pyfunction]
#[pyo3(signature = (agent, message, user_id="default_user", session_id="default_session", app_name="adk_app"))]
pub fn run_agent<'py>(
    py: Python<'py>,
    agent: &PyLlmAgent,
    message: String,
    user_id: &str,
    session_id: &str,
    app_name: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let agent = agent.inner.clone();
    let user_id = user_id.to_string();
    let session_id = session_id.to_string();
    let app_name = app_name.to_string();

    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let user_content = adk_core::Content::new("user").with_text(&message);
        let session_service = Arc::new(adk_session::InMemorySessionService::new());

        // Create session first (required by runner)
        session_service
            .create(adk_session::CreateRequest {
                app_name: app_name.clone(),
                user_id: user_id.clone(),
                session_id: Some(session_id.clone()),
                state: Default::default(),
            })
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let config = adk_runner::RunnerConfig {
            app_name,
            agent,
            session_service,
            artifact_service: None,
            memory_service: None,
            run_config: None,
        };

        let runner = adk_runner::Runner::new(config)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let stream_result = runner.run(user_id, session_id, user_content).await;
        let mut stream =
            stream_result.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let mut final_text = String::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(event) => {
                    if event.is_final_response() {
                        if let Some(content) = event.content() {
                            for part in content.parts.iter() {
                                if let Some(text) = part.text() {
                                    final_text.push_str(text);
                                }
                            }
                        }
                    }
                }
                Err(e) => return Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        }

        Ok(final_text)
    })
}
