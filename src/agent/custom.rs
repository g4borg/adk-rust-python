//! CustomAgent bindings - user-defined agent logic

use adk_core::{
    Agent, Content, Event, EventStream, InvocationContext, LlmResponse, Result as AdkResult,
};
use async_trait::async_trait;
use futures::stream;
use pyo3::prelude::*;
use std::sync::Arc;

use crate::context::PyInvocationContext;
use crate::types::PyContent;

/// Internal handler that wraps a Python async function
struct PythonAgentHandler {
    handler: Py<PyAny>,
}

unsafe impl Send for PythonAgentHandler {}
unsafe impl Sync for PythonAgentHandler {}

impl Clone for PythonAgentHandler {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            handler: self.handler.clone_ref(py),
        })
    }
}

/// Internal agent that uses a Python handler
struct PythonCustomAgent {
    name: String,
    description: String,
    sub_agents: Vec<Arc<dyn Agent>>,
    handler: PythonAgentHandler,
}

#[async_trait]
impl Agent for PythonCustomAgent {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn sub_agents(&self) -> &[Arc<dyn Agent>] {
        &self.sub_agents
    }

    async fn run(&self, ctx: Arc<dyn InvocationContext>) -> AdkResult<EventStream> {
        let handler = Python::with_gil(|py| self.handler.handler.clone_ref(py));
        let agent_name = self.name.clone();
        let invocation_id = ctx.invocation_id().to_string();

        // Create Python context
        let py_ctx = PyInvocationContext::from_invocation_context(ctx.as_ref());

        // Call Python handler in a blocking task to handle GIL
        let result = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                // Call the Python handler
                let result = handler.call1(py, (py_ctx,))?;

                // Check if result is a coroutine (async function)
                let asyncio = py.import_bound("asyncio")?;
                let is_coro = asyncio
                    .call_method1("iscoroutine", (&result,))?
                    .is_truthy()?;

                let final_result = if is_coro {
                    // Run the coroutine using asyncio.run()
                    asyncio.call_method1("run", (&result,))?
                } else {
                    result.into_bound(py)
                };

                // Extract string result - handler returns str or Content
                let text: String = if let Ok(s) = final_result.extract::<String>() {
                    s
                } else if let Ok(content) = final_result.extract::<PyContent>() {
                    // Get text from content by iterating parts
                    content.extract_text()
                } else {
                    // Try to convert to string
                    final_result.str()?.to_string()
                };

                Ok::<String, PyErr>(text)
            })
        })
        .await
        .map_err(|e| adk_core::AdkError::Agent(format!("Handler task failed: {}", e)))?
        .map_err(|e: PyErr| adk_core::AdkError::Agent(format!("Python handler error: {}", e)))?;

        // Create an event with the result using the Event::new helper
        let mut event = Event::new(&invocation_id);
        event.author = agent_name;
        event.llm_response = LlmResponse {
            content: Some(Content {
                role: "model".to_string(),
                parts: vec![adk_core::Part::Text { text: result }],
            }),
            partial: false,
            turn_complete: true,
            ..Default::default()
        };

        Ok(Box::pin(stream::once(async move { Ok(event) })))
    }
}

/// A custom agent with user-defined logic (not using an LLM)
///
/// Use the builder pattern to create a CustomAgent:
/// ```python
/// async def my_handler(ctx: InvocationContext) -> str:
///     return f"Hello, {ctx.user_id}!"
///
/// agent = (CustomAgent.builder("greeter")
///     .description("A friendly greeter")
///     .handler(my_handler)
///     .build())
/// ```
#[pyclass(name = "CustomAgent")]
pub struct PyCustomAgent {
    pub(crate) inner: Arc<dyn Agent>,
}

#[pymethods]
impl PyCustomAgent {
    /// Create a new CustomAgent builder
    #[staticmethod]
    fn builder(name: String) -> PyCustomAgentBuilder {
        PyCustomAgentBuilder::new(name)
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    #[getter]
    fn description(&self) -> String {
        self.inner.description().to_string()
    }

    fn __repr__(&self) -> String {
        format!("CustomAgent(name='{}')", self.name())
    }
}

/// Builder for creating CustomAgent instances
#[pyclass(name = "CustomAgentBuilder")]
pub struct PyCustomAgentBuilder {
    name: String,
    description: Option<String>,
    handler: Option<PythonAgentHandler>,
    sub_agents: Vec<Arc<dyn Agent>>,
}

#[pymethods]
impl PyCustomAgentBuilder {
    #[new]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            handler: None,
            sub_agents: Vec::new(),
        }
    }

    /// Set the agent description
    fn description(mut slf: PyRefMut<'_, Self>, desc: String) -> PyRefMut<'_, Self> {
        slf.description = Some(desc);
        slf
    }

    /// Set the handler function
    ///
    /// The handler should be an async function with signature:
    /// `async def handler(ctx: InvocationContext) -> str`
    ///
    /// Or a sync function:
    /// `def handler(ctx: InvocationContext) -> str`
    fn handler(mut slf: PyRefMut<'_, Self>, handler: Py<PyAny>) -> PyRefMut<'_, Self> {
        slf.handler = Some(PythonAgentHandler { handler });
        slf
    }

    /// Add a sub-agent (for orchestration)
    fn sub_agent<'a>(
        mut slf: PyRefMut<'a, Self>,
        agent: &Bound<'a, PyAny>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        // Try to extract as different agent types
        if let Ok(llm_agent) = agent.extract::<PyRef<'_, crate::agent::PyLlmAgent>>() {
            slf.sub_agents.push(llm_agent.inner.clone());
        } else if let Ok(custom_agent) = agent.extract::<PyRef<'_, PyCustomAgent>>() {
            slf.sub_agents.push(custom_agent.inner.clone());
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "sub_agent must be an LlmAgent or CustomAgent",
            ));
        }
        Ok(slf)
    }

    /// Build the CustomAgent
    fn build(&self) -> PyResult<PyCustomAgent> {
        let handler = self.handler.clone().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("CustomAgent requires a handler")
        })?;

        let agent = PythonCustomAgent {
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_default(),
            sub_agents: self.sub_agents.clone(),
            handler,
        };

        Ok(PyCustomAgent {
            inner: Arc::new(agent),
        })
    }
}
