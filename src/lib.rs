//! Python bindings for ADK-Rust
//!
//! Build AI agents in Python powered by Rust

use pyo3::prelude::*;

pub mod agent;
pub mod context;
pub mod error;
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
    m.add_class::<PySequentialAgent>()?;
    m.add_class::<PyParallelAgent>()?;
    m.add_class::<PyLoopAgent>()?;

    // Tools
    m.add_class::<PyFunctionTool>()?;
    m.add_class::<PyBasicToolset>()?;
    m.add_class::<PyExitLoopTool>()?;
    m.add_class::<PyLoadArtifactsTool>()?;
    m.add_class::<PyGoogleSearchTool>()?;

    // Session
    m.add_class::<PyInMemorySessionService>()?;
    m.add_class::<PyState>()?;
    m.add_class::<PyRunConfig>()?;
    m.add_class::<PyStreamingMode>()?;
    m.add_class::<PyCreateSessionRequest>()?;
    m.add_class::<PyGetSessionRequest>()?;

    // Runner
    m.add_class::<PyRunner>()?;
    m.add_function(wrap_pyfunction!(run_agent, m)?)?;

    // Error
    m.add_class::<error::PyAdkError>()?;

    Ok(())
}
