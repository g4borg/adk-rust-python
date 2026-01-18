//! AgentTool - Use agents as callable tools

use adk_core::Tool;
use adk_tool::{AgentTool, AgentToolConfig};
use pyo3::prelude::*;
use std::sync::Arc;

use crate::agent::{PyCustomAgent, PyLlmAgent};

/// Wrap an agent as a callable tool
///
/// AgentTool allows using an agent as a tool that can be called by other agents.
/// This enables powerful composition patterns where a coordinator agent can
/// invoke specialized sub-agents.
///
/// Example:
/// ```python
/// # Create a specialized agent
/// math_agent = (LlmAgent.builder("math_expert")
///     .description("Solves mathematical problems")
///     .instruction("You are a math expert. Solve problems step by step.")
///     .model(model)
///     .build())
///
/// # Wrap it as a tool
/// math_tool = AgentTool(math_agent)
///
/// # Use in coordinator agent
/// coordinator = (LlmAgent.builder("coordinator")
///     .instruction("Help users by delegating to specialists")
///     .tool(math_tool)
///     .build())
/// ```
#[pyclass(name = "AgentTool")]
#[derive(Clone)]
pub struct PyAgentTool {
    pub(crate) inner: Arc<dyn Tool>,
}

#[pymethods]
impl PyAgentTool {
    /// Create an AgentTool from an agent
    ///
    /// Args:
    ///     agent: The agent to wrap (LlmAgent or CustomAgent)
    ///     skip_summarization: If True, return raw output without summarization
    ///     forward_artifacts: If True, sub-agent can access parent's artifacts
    ///     timeout_secs: Optional timeout in seconds for sub-agent execution
    #[new]
    #[pyo3(signature = (agent, skip_summarization=false, forward_artifacts=true, timeout_secs=None))]
    fn new(
        agent: &Bound<'_, PyAny>,
        skip_summarization: bool,
        forward_artifacts: bool,
        timeout_secs: Option<u64>,
    ) -> PyResult<Self> {
        // Extract the agent Arc from either LlmAgent or CustomAgent
        let agent_arc: Arc<dyn adk_core::Agent> =
            if let Ok(llm_agent) = agent.extract::<PyRef<'_, PyLlmAgent>>() {
                llm_agent.inner.clone()
            } else if let Ok(custom_agent) = agent.extract::<PyRef<'_, PyCustomAgent>>() {
                custom_agent.inner.clone()
            } else {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "agent must be an LlmAgent or CustomAgent",
                ));
            };

        let mut config = AgentToolConfig::default();
        config.skip_summarization = skip_summarization;
        config.forward_artifacts = forward_artifacts;
        if let Some(secs) = timeout_secs {
            config.timeout = Some(std::time::Duration::from_secs(secs));
        }

        let tool = AgentTool::with_config(agent_arc, config);

        Ok(Self {
            inner: Arc::new(tool),
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
        format!(
            "AgentTool(name='{}', description='{}')",
            self.name(),
            self.description()
        )
    }
}
