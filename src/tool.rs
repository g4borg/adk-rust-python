//! Tool wrappers for Python

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Arc;
use adk_core::{Tool, ToolContext, Result as AdkResult};
use async_trait::async_trait;

use crate::context::PyToolContext;

/// A tool that wraps a Python function
#[pyclass(name = "FunctionTool")]
#[derive(Clone)]
pub struct PyFunctionTool {
    pub(crate) inner: Arc<dyn Tool>,
}

struct PythonTool {
    name: String,
    description: String,
    handler: Py<PyAny>,
    parameters_schema: Option<serde_json::Value>,
}

unsafe impl Send for PythonTool {}
unsafe impl Sync for PythonTool {}

#[async_trait]
impl Tool for PythonTool {
    fn name(&self) -> &str { &self.name }
    fn description(&self) -> &str { &self.description }
    fn parameters_schema(&self) -> Option<serde_json::Value> { self.parameters_schema.clone() }

    async fn execute(&self, ctx: Arc<dyn ToolContext>, args: serde_json::Value) -> AdkResult<serde_json::Value> {
        let handler = Python::with_gil(|py| self.handler.clone_ref(py));

        let result = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                let py_ctx = PyToolContext {
                    base: crate::context::PyContext::from_readonly(ctx.as_ref()),
                    function_call_id: ctx.function_call_id().to_string(),
                };

                let py_args = pythonize::pythonize(py, &args)
                    .map_err(|e| adk_core::AdkError::Tool(e.to_string()))?;

                let result = handler.call1(py, (py_ctx, py_args))
                    .map_err(|e| adk_core::AdkError::Tool(e.to_string()))?;

                pythonize::depythonize::<serde_json::Value>(result.bind(py))
                    .map_err(|e| adk_core::AdkError::Tool(e.to_string()))
            })
        })
        .await
        .map_err(|e| adk_core::AdkError::Tool(e.to_string()))??;

        Ok(result)
    }
}

#[pymethods]
impl PyFunctionTool {
    #[new]
    #[pyo3(signature = (name, description, handler, parameters_schema=None))]
    fn new(
        name: String,
        description: String,
        handler: Py<PyAny>,
        parameters_schema: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let schema = if let Some(schema_dict) = parameters_schema {
            Some(pythonize::depythonize::<serde_json::Value>(schema_dict.as_any())
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?)
        } else {
            None
        };

        let tool = PythonTool {
            name,
            description,
            handler,
            parameters_schema: schema,
        };

        Ok(Self { inner: Arc::new(tool) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    #[getter]
    fn description(&self) -> String { self.inner.description().to_string() }

    fn __repr__(&self) -> String {
        format!("FunctionTool(name='{}', description='{}')", self.name(), self.description())
    }
}

/// A collection of tools
#[pyclass(name = "BasicToolset")]
pub struct PyBasicToolset {
    name: String,
    tools: Vec<PyFunctionTool>,
}

#[pymethods]
impl PyBasicToolset {
    #[new]
    fn new(name: String) -> Self {
        Self { name, tools: Vec::new() }
    }

    fn add(&mut self, tool: PyFunctionTool) {
        self.tools.push(tool);
    }

    fn __len__(&self) -> usize { self.tools.len() }

    fn tools(&self) -> Vec<PyFunctionTool> { self.tools.clone() }

    #[getter]
    fn name(&self) -> String { self.name.clone() }
}

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
    fn name(&self) -> String { self.inner.name().to_string() }

    #[getter]
    fn description(&self) -> String { self.inner.description().to_string() }
}

/// Tool that loads artifacts
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
    fn name(&self) -> String { self.inner.name().to_string() }

    #[getter]
    fn description(&self) -> String { self.inner.description().to_string() }
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
    fn name(&self) -> String { self.inner.name().to_string() }

    #[getter]
    fn description(&self) -> String { self.inner.description().to_string() }
}
