//! LlmAgent and LlmAgentBuilder bindings

use pyo3::prelude::*;
use std::sync::Arc;

use crate::callbacks::{
    create_after_agent_callback, create_after_model_callback, create_after_tool_callback,
    create_before_agent_callback, create_before_model_callback, create_before_tool_callback,
};
use crate::model::extract_llm;
use crate::tool::PyFunctionTool;
use crate::tool::function::PyMcpToolWrapper;

/// An LLM-powered agent that uses language models for reasoning
#[pyclass(name = "LlmAgent")]
pub struct PyLlmAgent {
    pub(crate) inner: Arc<adk_agent::LlmAgent>,
}

#[pymethods]
impl PyLlmAgent {
    #[staticmethod]
    fn builder(name: String) -> PyLlmAgentBuilder {
        PyLlmAgentBuilder::new(name)
    }

    #[getter]
    fn name(&self) -> String {
        adk_core::Agent::name(self.inner.as_ref()).to_string()
    }

    #[getter]
    fn description(&self) -> String {
        adk_core::Agent::description(self.inner.as_ref()).to_string()
    }

    fn __repr__(&self) -> String {
        format!("LlmAgent(name='{}')", self.name())
    }
}

/// Builder for creating LlmAgent instances
#[pyclass(name = "LlmAgentBuilder")]
pub struct PyLlmAgentBuilder {
    name: String,
    description: Option<String>,
    instruction: Option<String>,
    model: Option<Arc<dyn adk_core::Llm>>,
    tools: Vec<Arc<dyn adk_core::Tool>>,
    sub_agents: Vec<Arc<dyn adk_core::Agent>>,
    output_key: Option<String>,
    // Store Python callbacks - we'll create Rust callbacks in build()
    before_agent_callbacks: Vec<Py<PyAny>>,
    after_agent_callbacks: Vec<Py<PyAny>>,
    before_model_callbacks: Vec<Py<PyAny>>,
    after_model_callbacks: Vec<Py<PyAny>>,
    before_tool_callbacks: Vec<Py<PyAny>>,
    after_tool_callbacks: Vec<Py<PyAny>>,
}

#[pymethods]
impl PyLlmAgentBuilder {
    #[new]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            instruction: None,
            model: None,
            tools: Vec::new(),
            sub_agents: Vec::new(),
            output_key: None,
            before_agent_callbacks: Vec::new(),
            after_agent_callbacks: Vec::new(),
            before_model_callbacks: Vec::new(),
            after_model_callbacks: Vec::new(),
            before_tool_callbacks: Vec::new(),
            after_tool_callbacks: Vec::new(),
        }
    }

    fn description(mut slf: PyRefMut<'_, Self>, desc: String) -> PyRefMut<'_, Self> {
        slf.description = Some(desc);
        slf
    }

    fn instruction(mut slf: PyRefMut<'_, Self>, instruction: String) -> PyRefMut<'_, Self> {
        slf.instruction = Some(instruction);
        slf
    }

    fn model<'a>(
        mut slf: PyRefMut<'a, Self>,
        model: &Bound<'a, PyAny>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let llm = extract_llm(model)?;
        slf.model = Some(llm);
        Ok(slf)
    }

    /// Add a FunctionTool to the agent
    fn tool<'a>(
        mut slf: PyRefMut<'a, Self>,
        tool: PyRef<'a, PyFunctionTool>,
    ) -> PyRefMut<'a, Self> {
        slf.tools.push(tool.inner.clone());
        slf
    }

    /// Add an MCP tool (from McpToolset.get_tools()) to the agent
    fn mcp_tool<'a>(mut slf: PyRefMut<'a, Self>, tool: &PyMcpToolWrapper) -> PyRefMut<'a, Self> {
        slf.tools.push(tool.inner.clone());
        slf
    }

    fn sub_agent<'a>(
        mut slf: PyRefMut<'a, Self>,
        agent: PyRef<'a, PyLlmAgent>,
    ) -> PyRefMut<'a, Self> {
        slf.sub_agents.push(agent.inner.clone());
        slf
    }

    fn output_key(mut slf: PyRefMut<'_, Self>, key: String) -> PyRefMut<'_, Self> {
        slf.output_key = Some(key);
        slf
    }

    /// Add a callback that runs before the agent executes.
    ///
    /// The callback receives a CallbackContext and can return:
    /// - None to continue normally
    /// - A Content or string to skip execution and return that response
    fn before_agent_callback(
        mut slf: PyRefMut<'_, Self>,
        callback: Py<PyAny>,
    ) -> PyRefMut<'_, Self> {
        slf.before_agent_callbacks.push(callback);
        slf
    }

    /// Add a callback that runs after the agent executes.
    ///
    /// The callback receives a CallbackContext and can return:
    /// - None to continue normally
    /// - A Content or string to modify the response
    fn after_agent_callback(
        mut slf: PyRefMut<'_, Self>,
        callback: Py<PyAny>,
    ) -> PyRefMut<'_, Self> {
        slf.after_agent_callbacks.push(callback);
        slf
    }

    /// Add a callback that runs before each model call.
    ///
    /// The callback receives (CallbackContext, LlmRequest) and can return:
    /// - None or BeforeModelResult.cont() to continue with the request
    /// - BeforeModelResult.skip(text) to skip the model call and use that response
    fn before_model_callback(
        mut slf: PyRefMut<'_, Self>,
        callback: Py<PyAny>,
    ) -> PyRefMut<'_, Self> {
        slf.before_model_callbacks.push(callback);
        slf
    }

    /// Add a callback that runs after each model call.
    ///
    /// The callback receives (CallbackContext, LlmResponse) and can return:
    /// - None to keep the original response
    /// - A modified LlmResponse to replace it
    fn after_model_callback(
        mut slf: PyRefMut<'_, Self>,
        callback: Py<PyAny>,
    ) -> PyRefMut<'_, Self> {
        slf.after_model_callbacks.push(callback);
        slf
    }

    /// Add a callback that runs before each tool call.
    ///
    /// The callback receives a CallbackContext and can return:
    /// - None to continue normally
    /// - A Content or string to skip the tool and return that response
    fn before_tool_callback(
        mut slf: PyRefMut<'_, Self>,
        callback: Py<PyAny>,
    ) -> PyRefMut<'_, Self> {
        slf.before_tool_callbacks.push(callback);
        slf
    }

    /// Add a callback that runs after each tool call.
    ///
    /// The callback receives a CallbackContext and can return:
    /// - None to continue normally
    /// - A Content or string to modify the tool response
    fn after_tool_callback(mut slf: PyRefMut<'_, Self>, callback: Py<PyAny>) -> PyRefMut<'_, Self> {
        slf.after_tool_callbacks.push(callback);
        slf
    }

    fn build(&self) -> PyResult<PyLlmAgent> {
        let model = self
            .model
            .clone()
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Model is required"))?;

        let mut builder = adk_agent::LlmAgentBuilder::new(&self.name).model(model);

        if let Some(ref desc) = self.description {
            builder = builder.description(desc);
        }
        if let Some(ref inst) = self.instruction {
            builder = builder.instruction(inst);
        }
        if let Some(ref key) = self.output_key {
            builder = builder.output_key(key);
        }

        for tool in &self.tools {
            builder = builder.tool(tool.clone());
        }
        for agent in &self.sub_agents {
            builder = builder.sub_agent(agent.clone());
        }

        // Add callbacks - convert Python callbacks to Rust callbacks
        for cb in &self.before_agent_callbacks {
            let cb_clone = Python::with_gil(|py| cb.clone_ref(py));
            builder = builder.before_callback(create_before_agent_callback(cb_clone));
        }
        for cb in &self.after_agent_callbacks {
            let cb_clone = Python::with_gil(|py| cb.clone_ref(py));
            builder = builder.after_callback(create_after_agent_callback(cb_clone));
        }
        for cb in &self.before_model_callbacks {
            let cb_clone = Python::with_gil(|py| cb.clone_ref(py));
            builder = builder.before_model_callback(create_before_model_callback(cb_clone));
        }
        for cb in &self.after_model_callbacks {
            let cb_clone = Python::with_gil(|py| cb.clone_ref(py));
            builder = builder.after_model_callback(create_after_model_callback(cb_clone));
        }
        for cb in &self.before_tool_callbacks {
            let cb_clone = Python::with_gil(|py| cb.clone_ref(py));
            builder = builder.before_tool_callback(create_before_tool_callback(cb_clone));
        }
        for cb in &self.after_tool_callbacks {
            let cb_clone = Python::with_gil(|py| cb.clone_ref(py));
            builder = builder.after_tool_callback(create_after_tool_callback(cb_clone));
        }

        let agent = builder
            .build()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyLlmAgent {
            inner: Arc::new(agent),
        })
    }
}
