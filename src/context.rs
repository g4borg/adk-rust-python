//! Context wrappers for Python

use pyo3::prelude::*;

use crate::session::PyState;
use crate::types::PyContent;

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

/// Invocation context passed to CustomAgent handlers
///
/// Provides access to session state, user content, and agent info.
#[pyclass(name = "InvocationContext")]
#[derive(Clone)]
pub struct PyInvocationContext {
    pub(crate) base: PyContext,
    pub(crate) user_content: Option<PyContent>,
    pub(crate) state: PyState,
}

#[pymethods]
impl PyInvocationContext {
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

    /// The user's message content that triggered this invocation
    #[getter]
    fn user_content(&self) -> Option<PyContent> {
        self.user_content.clone()
    }

    /// Session state - can read values set by previous turns
    #[getter]
    fn state(&self) -> PyState {
        self.state.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "InvocationContext(agent='{}', user='{}', session='{}')",
            self.base.agent_name, self.base.user_id, self.base.session_id
        )
    }
}

impl PyInvocationContext {
    /// Create from an InvocationContext trait object
    pub fn from_invocation_context(ctx: &dyn adk_core::InvocationContext) -> Self {
        // Get user content from session's conversation history (last user message)
        let user_content = ctx
            .session()
            .conversation_history()
            .into_iter()
            .rev()
            .find(|c| c.role == "user")
            .map(PyContent::from);

        // Build state from session state
        let state = PyState::from_session_state(ctx.session().state());

        Self {
            base: PyContext::from_readonly(ctx),
            user_content,
            state,
        }
    }
}

/// Callback context passed to before/after callbacks
///
/// Provides access to session state, user content, and agent info.
#[pyclass(name = "CallbackContext")]
#[derive(Clone)]
pub struct PyCallbackContext {
    pub(crate) base: PyContext,
    pub(crate) user_content: Option<PyContent>,
    pub(crate) state: PyState,
}

#[pymethods]
impl PyCallbackContext {
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

    /// The user's message content that triggered this invocation
    #[getter]
    fn user_content(&self) -> Option<PyContent> {
        self.user_content.clone()
    }

    /// Session state - can read values set by previous turns
    #[getter]
    fn state(&self) -> PyState {
        self.state.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "CallbackContext(agent='{}', user='{}', session='{}')",
            self.base.agent_name, self.base.user_id, self.base.session_id
        )
    }
}

impl PyCallbackContext {
    /// Create from a CallbackContext trait object
    pub fn from_callback_context(ctx: &dyn adk_core::CallbackContext) -> Self {
        // Get user content from user_content() method (from ReadonlyContext)
        let user_content = Some(PyContent::from(ctx.user_content().clone()));

        // CallbackContext doesn't have session access, create empty state
        let state = PyState::empty();

        Self {
            base: PyContext::from_readonly(ctx),
            user_content,
            state,
        }
    }
}
