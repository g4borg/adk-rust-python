//! LlmAgent and LlmAgentBuilder bindings

use pyo3::prelude::*;
use std::sync::Arc;

use crate::model::extract_llm;
use crate::tool::PyFunctionTool;

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

    fn tool<'a>(
        mut slf: PyRefMut<'a, Self>,
        tool: PyRef<'a, PyFunctionTool>,
    ) -> PyRefMut<'a, Self> {
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

        let agent = builder
            .build()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        Ok(PyLlmAgent {
            inner: Arc::new(agent),
        })
    }
}
