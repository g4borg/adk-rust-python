//! Context wrappers for Python

use pyo3::prelude::*;

/// Read-only context passed to tools and callbacks
#[pyclass(name = "Context")]
#[derive(Clone)]
pub struct PyContext {
    pub(crate) invocation_id: String,
    pub(crate) agent_name: String,
    pub(crate) user_id: String,
    pub(crate) app_name: String,
    pub(crate) session_id: String,
}

#[pymethods]
impl PyContext {
    #[getter]
    fn invocation_id(&self) -> String {
        self.invocation_id.clone()
    }

    #[getter]
    fn agent_name(&self) -> String {
        self.agent_name.clone()
    }

    #[getter]
    fn user_id(&self) -> String {
        self.user_id.clone()
    }

    #[getter]
    fn app_name(&self) -> String {
        self.app_name.clone()
    }

    #[getter]
    fn session_id(&self) -> String {
        self.session_id.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Context(invocation_id='{}', agent='{}', user='{}', session='{}')",
            self.invocation_id, self.agent_name, self.user_id, self.session_id
        )
    }
}

impl PyContext {
    pub fn from_readonly(ctx: &dyn adk_core::ReadonlyContext) -> Self {
        Self {
            invocation_id: ctx.invocation_id().to_string(),
            agent_name: ctx.agent_name().to_string(),
            user_id: ctx.user_id().to_string(),
            app_name: ctx.app_name().to_string(),
            session_id: ctx.session_id().to_string(),
        }
    }
}

/// Tool context with additional capabilities
#[pyclass(name = "ToolContext")]
#[derive(Clone)]
pub struct PyToolContext {
    pub(crate) base: PyContext,
    pub(crate) function_call_id: String,
}

#[pymethods]
impl PyToolContext {
    #[getter]
    fn invocation_id(&self) -> String {
        self.base.invocation_id.clone()
    }

    #[getter]
    fn agent_name(&self) -> String {
        self.base.agent_name.clone()
    }

    #[getter]
    fn user_id(&self) -> String {
        self.base.user_id.clone()
    }

    #[getter]
    fn app_name(&self) -> String {
        self.base.app_name.clone()
    }

    #[getter]
    fn session_id(&self) -> String {
        self.base.session_id.clone()
    }

    #[getter]
    fn function_call_id(&self) -> String {
        self.function_call_id.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "ToolContext(function_call_id='{}', agent='{}', session='{}')",
            self.function_call_id, self.base.agent_name, self.base.session_id
        )
    }
}
