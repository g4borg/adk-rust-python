//! Conditional routing agents
//!
//! This module provides Python bindings for conditional routing agents:
//! - `ConditionalAgent` - Rule-based conditional routing
//! - `LlmConditionalAgent` - LLM-powered intelligent routing

use adk_core::{Agent, Content, Event, EventStream, InvocationContext, Llm, LlmRequest, Part};
use async_stream::stream;
use async_trait::async_trait;
use futures::StreamExt;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use crate::context::PyInvocationContext;
use crate::model::extract_llm;

// ============================================================================
// ConditionalAgent - Rule-based routing
// ============================================================================

/// Internal wrapper for Python condition function
struct PythonConditionFn {
    condition: Py<PyAny>,
}

unsafe impl Send for PythonConditionFn {}
unsafe impl Sync for PythonConditionFn {}

impl PythonConditionFn {
    fn evaluate(&self, ctx: &dyn InvocationContext) -> bool {
        Python::with_gil(|py| {
            let py_ctx = PyInvocationContext::from_invocation_context(ctx);
            match self.condition.call1(py, (py_ctx,)) {
                Ok(result) => result.extract::<bool>(py).unwrap_or(false),
                Err(_) => false,
            }
        })
    }
}

/// Internal Rust agent that wraps Python condition
struct PythonConditionalAgent {
    name: String,
    description: String,
    condition: PythonConditionFn,
    if_agent: Arc<dyn Agent>,
    else_agent: Option<Arc<dyn Agent>>,
}

#[async_trait]
impl Agent for PythonConditionalAgent {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn sub_agents(&self) -> &[Arc<dyn Agent>] {
        &[]
    }

    async fn run(&self, ctx: Arc<dyn InvocationContext>) -> adk_core::Result<EventStream> {
        // Evaluate condition synchronously (it's a simple Python call)
        let condition_result = {
            let ctx_ref = ctx.as_ref();
            self.condition.evaluate(ctx_ref)
        };

        let agent = if condition_result {
            self.if_agent.clone()
        } else if let Some(else_agent) = &self.else_agent {
            else_agent.clone()
        } else {
            return Ok(Box::pin(futures::stream::empty()));
        };

        agent.run(ctx).await
    }
}

/// Rule-based conditional routing agent.
///
/// Routes execution to one of two agents based on a Python condition function
/// that evaluates session state, flags, or other deterministic criteria.
///
/// For LLM-based intelligent routing, use `LlmConditionalAgent` instead.
#[pyclass(name = "ConditionalAgent")]
pub struct PyConditionalAgent {
    pub(crate) inner: Arc<dyn Agent>,
}

#[pymethods]
impl PyConditionalAgent {
    /// Create a new ConditionalAgent.
    ///
    /// Args:
    ///     name: The agent name
    ///     condition: A function that takes InvocationContext and returns bool
    ///     if_agent: Agent to run when condition is True
    ///     else_agent: Optional agent to run when condition is False
    ///     description: Optional description
    #[new]
    #[pyo3(signature = (name, condition, if_agent, else_agent=None, description=None))]
    fn new(
        name: String,
        condition: Py<PyAny>,
        if_agent: &Bound<'_, PyAny>,
        else_agent: Option<&Bound<'_, PyAny>>,
        description: Option<String>,
    ) -> PyResult<Self> {
        // Extract if_agent
        let if_agent_arc = extract_agent_arc(if_agent)?;

        // Extract else_agent if provided
        let else_agent_arc = else_agent.map(extract_agent_arc).transpose()?;

        let agent = PythonConditionalAgent {
            name,
            description: description.unwrap_or_default(),
            condition: PythonConditionFn { condition },
            if_agent: if_agent_arc,
            else_agent: else_agent_arc,
        };

        Ok(Self {
            inner: Arc::new(agent),
        })
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
        format!("ConditionalAgent(name='{}')", self.name())
    }
}

// ============================================================================
// LlmConditionalAgent - LLM-powered intelligent routing
// ============================================================================

/// Internal Rust agent for LLM-based routing
struct PythonLlmConditionalAgent {
    name: String,
    description: String,
    model: Arc<dyn Llm>,
    instruction: String,
    routes: HashMap<String, Arc<dyn Agent>>,
    default_agent: Option<Arc<dyn Agent>>,
}

#[async_trait]
impl Agent for PythonLlmConditionalAgent {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn sub_agents(&self) -> &[Arc<dyn Agent>] {
        &[]
    }

    async fn run(&self, ctx: Arc<dyn InvocationContext>) -> adk_core::Result<EventStream> {
        let model = self.model.clone();
        let instruction = self.instruction.clone();
        let routes = self.routes.clone();
        let default_agent = self.default_agent.clone();
        let invocation_id = ctx.invocation_id().to_string();
        let agent_name = self.name.clone();

        let s = stream! {
            // Build classification request
            let user_content = ctx.user_content().clone();
            let user_text: String = user_content.parts.iter()
                .filter_map(|p| if let Part::Text { text } = p { Some(text.as_str()) } else { None })
                .collect::<Vec<_>>()
                .join(" ");

            let classification_prompt = format!(
                "{}\n\nUser input: {}",
                instruction,
                user_text
            );

            let request = LlmRequest {
                model: model.name().to_string(),
                contents: vec![Content::new("user").with_text(&classification_prompt)],
                tools: HashMap::new(),
                config: None,
            };

            // Call LLM for classification
            let mut response_stream = match model.generate_content(request, false).await {
                Ok(stream) => stream,
                Err(e) => {
                    yield Err(e);
                    return;
                }
            };

            // Collect classification response
            let mut classification = String::new();
            while let Some(chunk_result) = response_stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        if let Some(content) = chunk.content {
                            for part in content.parts {
                                if let Part::Text { text } = part {
                                    classification.push_str(&text);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                        return;
                    }
                }
            }

            // Normalize classification
            let classification = classification.trim().to_lowercase();

            // Emit routing event
            let mut routing_event = Event::new(&invocation_id);
            routing_event.author = agent_name.clone();
            routing_event.llm_response.content = Some(
                Content::new("model").with_text(format!("[Routing to: {}]", classification))
            );
            yield Ok(routing_event);

            // Find matching route
            let target_agent = routes.iter()
                .find(|(label, _)| classification.contains(label.as_str()))
                .map(|(_, agent)| agent.clone())
                .or(default_agent);

            // Execute target agent
            if let Some(agent) = target_agent {
                match agent.run(ctx.clone()).await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            yield event;
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                    }
                }
            } else {
                // No matching route and no default
                let mut error_event = Event::new(&invocation_id);
                error_event.author = agent_name;
                error_event.llm_response.content = Some(
                    Content::new("model").with_text(format!(
                        "No route found for classification '{}'. Available routes: {:?}",
                        classification,
                        routes.keys().collect::<Vec<_>>()
                    ))
                );
                yield Ok(error_event);
            }
        };

        Ok(Box::pin(s))
    }
}

/// LLM-based intelligent routing agent.
///
/// Uses an LLM to classify user input and route to the appropriate sub-agent
/// based on the classification result. Supports multi-way routing.
#[pyclass(name = "LlmConditionalAgent")]
pub struct PyLlmConditionalAgent {
    pub(crate) inner: Arc<dyn Agent>,
}

#[pymethods]
impl PyLlmConditionalAgent {
    /// Create a builder for LlmConditionalAgent.
    #[staticmethod]
    fn builder(name: String, model: &Bound<'_, PyAny>) -> PyResult<PyLlmConditionalAgentBuilder> {
        let model_arc = extract_llm(model)?;
        Ok(PyLlmConditionalAgentBuilder::new(name, model_arc))
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
        format!("LlmConditionalAgent(name='{}')", self.name())
    }
}

/// Builder for LlmConditionalAgent.
#[pyclass(name = "LlmConditionalAgentBuilder")]
pub struct PyLlmConditionalAgentBuilder {
    name: String,
    description: Option<String>,
    model: Arc<dyn Llm>,
    instruction: Option<String>,
    routes: HashMap<String, Arc<dyn Agent>>,
    default_agent: Option<Arc<dyn Agent>>,
}

impl PyLlmConditionalAgentBuilder {
    fn new(name: String, model: Arc<dyn Llm>) -> Self {
        Self {
            name,
            description: None,
            model,
            instruction: None,
            routes: HashMap::new(),
            default_agent: None,
        }
    }
}

#[pymethods]
impl PyLlmConditionalAgentBuilder {
    /// Set a description for the agent.
    fn description(mut slf: PyRefMut<'_, Self>, desc: String) -> PyRefMut<'_, Self> {
        slf.description = Some(desc);
        slf
    }

    /// Set the classification instruction.
    ///
    /// The instruction should tell the LLM to classify the user's input
    /// and respond with ONLY the category name (matching a route key).
    fn instruction(mut slf: PyRefMut<'_, Self>, instruction: String) -> PyRefMut<'_, Self> {
        slf.instruction = Some(instruction);
        slf
    }

    /// Add a route mapping a classification label to an agent.
    ///
    /// When the LLM's response contains this label, execution transfers
    /// to the specified agent.
    fn route<'a>(
        mut slf: PyRefMut<'a, Self>,
        label: String,
        agent: &Bound<'a, PyAny>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let agent_arc = extract_agent_arc(agent)?;
        slf.routes.insert(label.to_lowercase(), agent_arc);
        Ok(slf)
    }

    /// Set the default agent to use when no route matches.
    fn default_route<'a>(
        mut slf: PyRefMut<'a, Self>,
        agent: &Bound<'a, PyAny>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let agent_arc = extract_agent_arc(agent)?;
        slf.default_agent = Some(agent_arc);
        Ok(slf)
    }

    /// Build the LlmConditionalAgent.
    fn build(&self) -> PyResult<PyLlmConditionalAgent> {
        let instruction = self.instruction.clone().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(
                "Instruction is required for LlmConditionalAgent",
            )
        })?;

        if self.routes.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "At least one route is required for LlmConditionalAgent",
            ));
        }

        let agent = PythonLlmConditionalAgent {
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_default(),
            model: self.model.clone(),
            instruction,
            routes: self.routes.clone(),
            default_agent: self.default_agent.clone(),
        };

        Ok(PyLlmConditionalAgent {
            inner: Arc::new(agent),
        })
    }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Extract an Arc<dyn Agent> from a Python agent object.
fn extract_agent_arc(agent: &Bound<'_, PyAny>) -> PyResult<Arc<dyn Agent>> {
    use crate::agent::custom::PyCustomAgent;
    use crate::agent::llm::PyLlmAgent;

    // Try LlmAgent first
    if let Ok(llm_agent) = agent.extract::<PyRef<'_, PyLlmAgent>>() {
        return Ok(llm_agent.inner.clone());
    }

    // Try CustomAgent
    if let Ok(custom_agent) = agent.extract::<PyRef<'_, PyCustomAgent>>() {
        return Ok(custom_agent.inner.clone());
    }

    // Try ConditionalAgent
    if let Ok(cond_agent) = agent.extract::<PyRef<'_, PyConditionalAgent>>() {
        return Ok(cond_agent.inner.clone());
    }

    // Try LlmConditionalAgent
    if let Ok(llm_cond_agent) = agent.extract::<PyRef<'_, PyLlmConditionalAgent>>() {
        return Ok(llm_cond_agent.inner.clone());
    }

    // Try SequentialAgent
    if let Ok(seq_agent) = agent.extract::<PyRef<'_, crate::agent::workflow::PySequentialAgent>>() {
        return Ok(seq_agent.inner.clone() as Arc<dyn Agent>);
    }

    // Try ParallelAgent
    if let Ok(par_agent) = agent.extract::<PyRef<'_, crate::agent::workflow::PyParallelAgent>>() {
        return Ok(par_agent.inner.clone() as Arc<dyn Agent>);
    }

    // Try LoopAgent
    if let Ok(loop_agent) = agent.extract::<PyRef<'_, crate::agent::workflow::PyLoopAgent>>() {
        return Ok(loop_agent.inner.clone() as Arc<dyn Agent>);
    }

    Err(pyo3::exceptions::PyTypeError::new_err(
        "Expected an agent (LlmAgent, CustomAgent, ConditionalAgent, LlmConditionalAgent, SequentialAgent, ParallelAgent, or LoopAgent)",
    ))
}
