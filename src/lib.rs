//! Python bindings for ADK-Rust
//!
//! Build AI agents in Python powered by Rust.
//!
//! ## Module Structure
//!
//! - `agent` - Agent types (LlmAgent, CustomAgent, workflow agents)
//! - `model` - LLM providers (Gemini, OpenAI, Anthropic, etc.)
//! - `tool` - Tool system (FunctionTool, built-in tools)
//! - `session` - Session and state management
//! - `runner` - Agent execution
//! - `types` - Core types (Content, Part, Event)
//! - `context` - Execution context types
//! - `error` - Error types

use pyo3::prelude::*;

pub mod agent;
pub mod artifact;
pub mod callbacks;
pub mod context;
pub mod error;
pub mod guardrail;
pub mod memory;
pub mod model;
pub mod runner;
pub mod session;
pub mod tool;
pub mod types;

use agent::*;
use model::*;
use runner::*;
use session::*;
use tool::*;
use types::*;

/// ADK-Rust Python module
#[pymodule]
fn _adk_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Types
    m.add_class::<PyContent>()?;
    m.add_class::<PyPart>()?;
    m.add_class::<PyEvent>()?;

    // Models
    m.add_class::<PyGeminiModel>()?;
    m.add_class::<PyOpenAIModel>()?;
    m.add_class::<PyAnthropicModel>()?;
    m.add_class::<PyDeepSeekModel>()?;
    m.add_class::<PyGroqModel>()?;
    m.add_class::<PyOllamaModel>()?;
    m.add_class::<PyMockLlm>()?;

    // Agents
    m.add_class::<PyLlmAgent>()?;
    m.add_class::<PyLlmAgentBuilder>()?;
    m.add_class::<PyCustomAgent>()?;
    m.add_class::<PyCustomAgentBuilder>()?;
    m.add_class::<PySequentialAgent>()?;
    m.add_class::<PyParallelAgent>()?;
    m.add_class::<PyLoopAgent>()?;
    m.add_class::<agent::PyConditionalAgent>()?;
    m.add_class::<agent::PyLlmConditionalAgent>()?;
    m.add_class::<agent::PyLlmConditionalAgentBuilder>()?;

    // Tools
    m.add_class::<PyFunctionTool>()?;
    m.add_class::<PyBasicToolset>()?;
    m.add_class::<PyExitLoopTool>()?;
    m.add_class::<PyLoadArtifactsTool>()?;
    m.add_class::<PyGoogleSearchTool>()?;
    m.add_class::<tool::PyAgentTool>()?;
    m.add_class::<tool::PyMcpToolset>()?;
    m.add_class::<tool::PyMcpToolWrapper>()?;

    // Session
    m.add_class::<PyInMemorySessionService>()?;
    m.add_class::<PySession>()?;
    m.add_class::<PyState>()?;
    m.add_class::<PyRunConfig>()?;
    m.add_class::<PyStreamingMode>()?;
    m.add_class::<PyCreateSessionRequest>()?;
    m.add_class::<PyGetSessionRequest>()?;
    m.add_class::<PyListSessionRequest>()?;
    m.add_class::<PyDeleteSessionRequest>()?;
    m.add_class::<session::PyGenerateContentConfig>()?;

    // Runner
    m.add_class::<PyRunner>()?;
    m.add_class::<runner::PyEventStream>()?;
    m.add_function(wrap_pyfunction!(run_agent, m)?)?;

    // Context
    m.add_class::<context::PyContext>()?;
    m.add_class::<context::PyToolContext>()?;
    m.add_class::<context::PyInvocationContext>()?;
    m.add_class::<context::PyCallbackContext>()?;

    // Callbacks
    m.add_class::<callbacks::PyLlmRequest>()?;
    m.add_class::<callbacks::PyLlmResponse>()?;
    m.add_class::<callbacks::PyBeforeModelResult>()?;

    // Error
    m.add_class::<error::PyAdkError>()?;

    // Guardrails
    m.add_class::<guardrail::PySeverity>()?;
    m.add_class::<guardrail::PyPiiType>()?;
    m.add_class::<guardrail::PyContentFilter>()?;
    m.add_class::<guardrail::PyPiiRedactor>()?;
    m.add_class::<guardrail::PyGuardrailSet>()?;
    m.add_class::<guardrail::PyGuardrailResult>()?;
    m.add_class::<guardrail::PyGuardrailFailure>()?;
    m.add_function(wrap_pyfunction!(guardrail::run_guardrails, m)?)?;

    // Memory
    m.add_class::<memory::PyMemoryEntry>()?;
    m.add_class::<memory::PyInMemoryMemoryService>()?;

    // Artifact
    m.add_class::<artifact::PyInMemoryArtifactService>()?;

    Ok(())
}
