//! Workflow agents: Sequential, Parallel, Loop

use pyo3::prelude::*;
use std::sync::Arc;

use super::llm::PyLlmAgent;

/// Executes agents in sequence, one after another
#[pyclass(name = "SequentialAgent")]
pub struct PySequentialAgent {
    pub(crate) inner: Arc<adk_agent::SequentialAgent>,
}

#[pymethods]
impl PySequentialAgent {
    #[new]
    fn new(name: String, agents: Vec<PyRef<'_, PyLlmAgent>>) -> Self {
        let rust_agents: Vec<Arc<dyn adk_core::Agent>> = agents
            .iter()
            .map(|a| a.inner.clone() as Arc<dyn adk_core::Agent>)
            .collect();

        Self {
            inner: Arc::new(adk_agent::SequentialAgent::new(&name, rust_agents)),
        }
    }

    #[getter]
    fn name(&self) -> String {
        adk_core::Agent::name(self.inner.as_ref()).to_string()
    }

    fn __repr__(&self) -> String {
        format!("SequentialAgent(name='{}')", self.name())
    }
}

/// Executes agents in parallel, concurrently
#[pyclass(name = "ParallelAgent")]
pub struct PyParallelAgent {
    pub(crate) inner: Arc<adk_agent::ParallelAgent>,
}

#[pymethods]
impl PyParallelAgent {
    #[new]
    fn new(name: String, agents: Vec<PyRef<'_, PyLlmAgent>>) -> Self {
        let rust_agents: Vec<Arc<dyn adk_core::Agent>> = agents
            .iter()
            .map(|a| a.inner.clone() as Arc<dyn adk_core::Agent>)
            .collect();

        Self {
            inner: Arc::new(adk_agent::ParallelAgent::new(&name, rust_agents)),
        }
    }

    #[getter]
    fn name(&self) -> String {
        adk_core::Agent::name(self.inner.as_ref()).to_string()
    }

    fn __repr__(&self) -> String {
        format!("ParallelAgent(name='{}')", self.name())
    }
}

/// Executes agents in a loop until a condition is met
#[pyclass(name = "LoopAgent")]
pub struct PyLoopAgent {
    pub(crate) inner: Arc<adk_agent::LoopAgent>,
}

#[pymethods]
impl PyLoopAgent {
    #[new]
    #[pyo3(signature = (name, agents, max_iterations=10))]
    fn new(name: String, agents: Vec<PyRef<'_, PyLlmAgent>>, max_iterations: u32) -> Self {
        let rust_agents: Vec<Arc<dyn adk_core::Agent>> = agents
            .iter()
            .map(|a| a.inner.clone() as Arc<dyn adk_core::Agent>)
            .collect();

        Self {
            inner: Arc::new(
                adk_agent::LoopAgent::new(&name, rust_agents).with_max_iterations(max_iterations),
            ),
        }
    }

    #[getter]
    fn name(&self) -> String {
        adk_core::Agent::name(self.inner.as_ref()).to_string()
    }

    fn __repr__(&self) -> String {
        format!("LoopAgent(name='{}', max_iterations=?)", self.name())
    }
}
