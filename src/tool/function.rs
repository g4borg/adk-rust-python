//! FunctionTool and BasicToolset - user-defined tools

use adk_core::{Result as AdkResult, Tool, ToolContext};
use async_trait::async_trait;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Arc;

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
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn parameters_schema(&self) -> Option<serde_json::Value> {
        self.parameters_schema.clone()
    }

    async fn execute(
        &self,
        ctx: Arc<dyn ToolContext>,
        args: serde_json::Value,
    ) -> AdkResult<serde_json::Value> {
        let handler = Python::with_gil(|py| self.handler.clone_ref(py));

        let result = tokio::task::spawn_blocking(move || {
            Python::with_gil(|py| {
                let py_ctx = PyToolContext {
                    base: crate::context::PyContext::from_readonly(ctx.as_ref()),
                    function_call_id: ctx.function_call_id().to_string(),
                };

                let py_args = pythonize::pythonize(py, &args)
                    .map_err(|e| adk_core::AdkError::Tool(e.to_string()))?;

                let result = handler
                    .call1(py, (py_ctx, py_args))
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
            Some(
                pythonize::depythonize::<serde_json::Value>(schema_dict.as_any())
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            )
        } else {
            None
        };

        let tool = PythonTool {
            name,
            description,
            handler,
            parameters_schema: schema,
        };

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
            "FunctionTool(name='{}', description='{}')",
            self.name(),
            self.description()
        )
    }
}

/// A wrapper for tools from MCP or other dynamic sources
/// Unlike FunctionTool which wraps a Python function, this wraps an existing Rust Tool
#[pyclass(name = "McpTool")]
#[derive(Clone)]
pub struct PyMcpToolWrapper {
    pub(crate) inner: Arc<dyn Tool>,
}

impl PyMcpToolWrapper {
    pub fn new(tool: Arc<dyn Tool>) -> Self {
        Self { inner: tool }
    }
}

#[pymethods]
impl PyMcpToolWrapper {
    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    #[getter]
    fn description(&self) -> String {
        self.inner.description().to_string()
    }

    #[getter]
    fn parameters_schema(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        if let Some(schema) = self.inner.parameters_schema() {
            let py_obj = pythonize::pythonize(py, &schema)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            Ok(Some(py_obj.into()))
        } else {
            Ok(None)
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "McpTool(name='{}', description='{}')",
            self.name(),
            self.description()
        )
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
        Self {
            name,
            tools: Vec::new(),
        }
    }

    fn add(&mut self, tool: PyFunctionTool) {
        self.tools.push(tool);
    }

    fn __len__(&self) -> usize {
        self.tools.len()
    }

    fn tools(&self) -> Vec<PyFunctionTool> {
        self.tools.clone()
    }

    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "BasicToolset(name='{}', tools={})",
            self.name,
            self.tools.len()
        )
    }
}
