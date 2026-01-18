//! Callback bindings for Python
//!
//! This module provides Python callback support for agent, model, and tool lifecycle hooks.
//! Callbacks can intercept and modify behavior at various stages of agent execution.

use adk_core::{
    AfterAgentCallback, AfterModelCallback, AfterToolCallback, BeforeAgentCallback,
    BeforeModelCallback, BeforeModelResult, BeforeToolCallback, CallbackContext, Content,
    LlmRequest, LlmResponse,
};
use pyo3::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::context::PyCallbackContext;
use crate::types::PyContent;

// ============================================================================
// Python callback wrapper types
// ============================================================================

/// Wrapper to hold a Python callback function safely across threads
pub struct PythonCallback {
    callback: Py<PyAny>,
}

unsafe impl Send for PythonCallback {}
unsafe impl Sync for PythonCallback {}

impl Clone for PythonCallback {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            callback: self.callback.clone_ref(py),
        })
    }
}

impl PythonCallback {
    pub fn new(callback: Py<PyAny>) -> Self {
        Self { callback }
    }

    /// Call the Python callback and return an optional Content
    fn call_for_content(&self, ctx: Arc<dyn CallbackContext>) -> Option<Content> {
        Python::with_gil(|py| {
            let py_ctx = PyCallbackContext::from_callback_context(ctx.as_ref());
            match self.callback.call1(py, (py_ctx,)) {
                Ok(result) => {
                    // Check if result is awaitable (coroutine)
                    let asyncio = py.import_bound("asyncio").ok()?;
                    let is_coro = asyncio
                        .call_method1("iscoroutine", (&result,))
                        .ok()?
                        .is_truthy()
                        .ok()?;

                    let final_result = if is_coro {
                        asyncio.call_method1("run", (&result,)).ok()?
                    } else {
                        result.into_bound(py)
                    };

                    // Return None if Python returned None
                    if final_result.is_none() {
                        return None;
                    }

                    // Try to extract as PyContent
                    if let Ok(content) = final_result.extract::<PyContent>() {
                        return Some(content.into());
                    }

                    // Try to extract as string and convert to Content
                    if let Ok(text) = final_result.extract::<String>() {
                        return Some(Content::new("model").with_text(&text));
                    }

                    None
                }
                Err(_) => None,
            }
        })
    }

    /// Call the Python before_model callback and return BeforeModelResult
    fn call_for_before_model(
        &self,
        ctx: Arc<dyn CallbackContext>,
        request: LlmRequest,
    ) -> BeforeModelResult {
        Python::with_gil(|py| {
            let py_ctx = PyCallbackContext::from_callback_context(ctx.as_ref());
            let py_request = PyLlmRequest::from(request.clone());

            match self.callback.call1(py, (py_ctx, py_request)) {
                Ok(result) => {
                    // Check if result is awaitable (coroutine)
                    let asyncio = match py.import_bound("asyncio") {
                        Ok(a) => a,
                        Err(_) => return BeforeModelResult::Continue(request),
                    };

                    let is_coro = match asyncio.call_method1("iscoroutine", (&result,)) {
                        Ok(r) => r.is_truthy().unwrap_or(false),
                        Err(_) => false,
                    };

                    let final_result = if is_coro {
                        match asyncio.call_method1("run", (&result,)) {
                            Ok(r) => r,
                            Err(_) => return BeforeModelResult::Continue(request),
                        }
                    } else {
                        result.into_bound(py)
                    };

                    // Return None/Continue if Python returned None
                    if final_result.is_none() {
                        return BeforeModelResult::Continue(request);
                    }

                    // Check if it's a BeforeModelResult
                    if let Ok(bmr) = final_result.extract::<PyBeforeModelResult>() {
                        return bmr.into_rust(request);
                    }

                    // If string returned, treat as skip with that response
                    if let Ok(text) = final_result.extract::<String>() {
                        let response = LlmResponse::new(Content::new("model").with_text(&text));
                        return BeforeModelResult::Skip(response);
                    }

                    BeforeModelResult::Continue(request)
                }
                Err(_) => BeforeModelResult::Continue(request),
            }
        })
    }

    /// Call the Python after_model callback and return optional modified LlmResponse
    fn call_for_after_model(
        &self,
        ctx: Arc<dyn CallbackContext>,
        response: LlmResponse,
    ) -> Option<LlmResponse> {
        Python::with_gil(|py| {
            let py_ctx = PyCallbackContext::from_callback_context(ctx.as_ref());
            let py_response = PyLlmResponse::from(response.clone());

            match self.callback.call1(py, (py_ctx, py_response)) {
                Ok(result) => {
                    // Check if result is awaitable (coroutine)
                    let asyncio = py.import_bound("asyncio").ok()?;
                    let is_coro = asyncio
                        .call_method1("iscoroutine", (&result,))
                        .ok()?
                        .is_truthy()
                        .ok()?;

                    let final_result = if is_coro {
                        asyncio.call_method1("run", (&result,)).ok()?
                    } else {
                        result.into_bound(py)
                    };

                    // Return None if Python returned None (no modification)
                    if final_result.is_none() {
                        return None;
                    }

                    // Try to extract modified response
                    if let Ok(py_resp) = final_result.extract::<PyLlmResponse>() {
                        return Some(py_resp.into());
                    }

                    None
                }
                Err(_) => None,
            }
        })
    }
}

// ============================================================================
// Factory functions to create Rust callbacks from Python functions
// ============================================================================

/// Create a BeforeAgentCallback from a Python function
pub fn create_before_agent_callback(py_callback: Py<PyAny>) -> BeforeAgentCallback {
    let wrapper = PythonCallback::new(py_callback);
    Box::new(
        move |ctx: Arc<dyn CallbackContext>| -> Pin<
            Box<dyn Future<Output = adk_core::Result<Option<Content>>> + Send>,
        > {
            let wrapper = wrapper.clone();
            Box::pin(async move {
                let result = tokio::task::spawn_blocking(move || wrapper.call_for_content(ctx))
                    .await
                    .map_err(|e| {
                        adk_core::AdkError::Agent(format!("Before agent callback failed: {}", e))
                    })?;
                Ok(result)
            })
        },
    )
}

/// Create an AfterAgentCallback from a Python function
pub fn create_after_agent_callback(py_callback: Py<PyAny>) -> AfterAgentCallback {
    let wrapper = PythonCallback::new(py_callback);
    Box::new(
        move |ctx: Arc<dyn CallbackContext>| -> Pin<
            Box<dyn Future<Output = adk_core::Result<Option<Content>>> + Send>,
        > {
            let wrapper = wrapper.clone();
            Box::pin(async move {
                let result = tokio::task::spawn_blocking(move || wrapper.call_for_content(ctx))
                    .await
                    .map_err(|e| {
                        adk_core::AdkError::Agent(format!("After agent callback failed: {}", e))
                    })?;
                Ok(result)
            })
        },
    )
}

/// Create a BeforeModelCallback from a Python function
pub fn create_before_model_callback(py_callback: Py<PyAny>) -> BeforeModelCallback {
    let wrapper = PythonCallback::new(py_callback);
    Box::new(
        move |ctx: Arc<dyn CallbackContext>,
              request: LlmRequest|
              -> Pin<Box<dyn Future<Output = adk_core::Result<BeforeModelResult>> + Send>> {
            let wrapper = wrapper.clone();
            Box::pin(async move {
                let result = tokio::task::spawn_blocking(move || {
                    wrapper.call_for_before_model(ctx, request)
                })
                .await
                .map_err(|e| {
                    adk_core::AdkError::Agent(format!("Before model callback failed: {}", e))
                })?;
                Ok(result)
            })
        },
    )
}

/// Create an AfterModelCallback from a Python function
pub fn create_after_model_callback(py_callback: Py<PyAny>) -> AfterModelCallback {
    let wrapper = PythonCallback::new(py_callback);
    Box::new(
        move |ctx: Arc<dyn CallbackContext>, response: LlmResponse| -> Pin<
            Box<dyn Future<Output = adk_core::Result<Option<LlmResponse>>> + Send>,
        > {
            let wrapper = wrapper.clone();
            Box::pin(async move {
                let result = tokio::task::spawn_blocking(move || {
                    wrapper.call_for_after_model(ctx, response)
                })
                .await
                .map_err(|e| {
                    adk_core::AdkError::Agent(format!("After model callback failed: {}", e))
                })?;
                Ok(result)
            })
        },
    )
}

/// Create a BeforeToolCallback from a Python function
pub fn create_before_tool_callback(py_callback: Py<PyAny>) -> BeforeToolCallback {
    let wrapper = PythonCallback::new(py_callback);
    Box::new(
        move |ctx: Arc<dyn CallbackContext>| -> Pin<
            Box<dyn Future<Output = adk_core::Result<Option<Content>>> + Send>,
        > {
            let wrapper = wrapper.clone();
            Box::pin(async move {
                let result = tokio::task::spawn_blocking(move || wrapper.call_for_content(ctx))
                    .await
                    .map_err(|e| {
                        adk_core::AdkError::Agent(format!("Before tool callback failed: {}", e))
                    })?;
                Ok(result)
            })
        },
    )
}

/// Create an AfterToolCallback from a Python function
pub fn create_after_tool_callback(py_callback: Py<PyAny>) -> AfterToolCallback {
    let wrapper = PythonCallback::new(py_callback);
    Box::new(
        move |ctx: Arc<dyn CallbackContext>| -> Pin<
            Box<dyn Future<Output = adk_core::Result<Option<Content>>> + Send>,
        > {
            let wrapper = wrapper.clone();
            Box::pin(async move {
                let result = tokio::task::spawn_blocking(move || wrapper.call_for_content(ctx))
                    .await
                    .map_err(|e| {
                        adk_core::AdkError::Agent(format!("After tool callback failed: {}", e))
                    })?;
                Ok(result)
            })
        },
    )
}

// ============================================================================
// Python-exposed types for callbacks
// ============================================================================

/// Python wrapper for LlmRequest
#[pyclass(name = "LlmRequest")]
#[derive(Clone)]
pub struct PyLlmRequest {
    #[pyo3(get)]
    pub model: String,
    #[pyo3(get)]
    pub contents: Vec<PyContent>,
}

impl From<LlmRequest> for PyLlmRequest {
    fn from(req: LlmRequest) -> Self {
        Self {
            model: req.model,
            contents: req.contents.into_iter().map(PyContent::from).collect(),
        }
    }
}

#[pymethods]
impl PyLlmRequest {
    fn __repr__(&self) -> String {
        format!(
            "LlmRequest(model='{}', contents_count={})",
            self.model,
            self.contents.len()
        )
    }
}

/// Python wrapper for LlmResponse
#[pyclass(name = "LlmResponse")]
#[derive(Clone)]
pub struct PyLlmResponse {
    content: Option<PyContent>,
    #[pyo3(get)]
    pub partial: bool,
    #[pyo3(get)]
    pub turn_complete: bool,
}

impl From<LlmResponse> for PyLlmResponse {
    fn from(resp: LlmResponse) -> Self {
        Self {
            content: resp.content.map(PyContent::from),
            partial: resp.partial,
            turn_complete: resp.turn_complete,
        }
    }
}

impl From<PyLlmResponse> for LlmResponse {
    fn from(resp: PyLlmResponse) -> Self {
        LlmResponse {
            content: resp.content.map(|c| c.into()),
            partial: resp.partial,
            turn_complete: resp.turn_complete,
            ..Default::default()
        }
    }
}

#[pymethods]
impl PyLlmResponse {
    #[new]
    #[pyo3(signature = (content=None, partial=false, turn_complete=true))]
    fn new(content: Option<PyContent>, partial: bool, turn_complete: bool) -> Self {
        Self {
            content,
            partial,
            turn_complete,
        }
    }

    #[getter]
    fn content(&self) -> Option<PyContent> {
        self.content.clone()
    }

    fn get_text(&self) -> Option<String> {
        self.content.as_ref().map(|c| c.extract_text())
    }

    fn __repr__(&self) -> String {
        format!(
            "LlmResponse(partial={}, turn_complete={})",
            self.partial, self.turn_complete
        )
    }
}

/// Python wrapper for BeforeModelResult
#[pyclass(name = "BeforeModelResult")]
#[derive(Clone)]
pub struct PyBeforeModelResult {
    skip: bool,
    response_text: Option<String>,
}

impl PyBeforeModelResult {
    fn into_rust(self, request: LlmRequest) -> BeforeModelResult {
        if self.skip {
            let response = if let Some(text) = self.response_text {
                LlmResponse::new(Content::new("model").with_text(&text))
            } else {
                LlmResponse::new(Content::new("model").with_text(""))
            };
            BeforeModelResult::Skip(response)
        } else {
            BeforeModelResult::Continue(request)
        }
    }
}

#[pymethods]
impl PyBeforeModelResult {
    /// Continue with the model call (possibly with modified request)
    #[staticmethod]
    fn cont() -> Self {
        Self {
            skip: false,
            response_text: None,
        }
    }

    /// Skip the model call and return the given response text
    #[staticmethod]
    fn skip(response_text: String) -> Self {
        Self {
            skip: true,
            response_text: Some(response_text),
        }
    }

    fn __repr__(&self) -> String {
        if self.skip {
            format!(
                "BeforeModelResult.skip('{}')",
                self.response_text.as_deref().unwrap_or("")
            )
        } else {
            "BeforeModelResult.cont()".to_string()
        }
    }
}
