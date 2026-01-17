//! Built-in tools: ExitLoop, LoadArtifacts, GoogleSearch

use adk_core::Tool;
use pyo3::prelude::*;
use std::sync::Arc;

/// Tool that exits a loop agent
#[pyclass(name = "ExitLoopTool")]
#[derive(Clone)]
pub struct PyExitLoopTool {
    pub(crate) inner: Arc<dyn Tool>,
}

#[pymethods]
impl PyExitLoopTool {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(adk_tool::ExitLoopTool::new()),
        }
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
        format!("ExitLoopTool(name='{}')", self.name())
    }
}

/// Tool that loads artifacts into the context
#[pyclass(name = "LoadArtifactsTool")]
#[derive(Clone)]
pub struct PyLoadArtifactsTool {
    pub(crate) inner: Arc<dyn Tool>,
}

#[pymethods]
impl PyLoadArtifactsTool {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(adk_tool::LoadArtifactsTool::new()),
        }
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
        format!("LoadArtifactsTool(name='{}')", self.name())
    }
}

/// Google Search tool (internal to Gemini)
#[pyclass(name = "GoogleSearchTool")]
#[derive(Clone)]
pub struct PyGoogleSearchTool {
    pub(crate) inner: Arc<dyn Tool>,
}

#[pymethods]
impl PyGoogleSearchTool {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(adk_tool::GoogleSearchTool::new()),
        }
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
        format!("GoogleSearchTool(name='{}')", self.name())
    }
}
