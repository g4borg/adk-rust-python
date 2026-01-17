//! Core types exposed to Python

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A message part - can be text, function call, or function response
#[pyclass(name = "Part")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PyPart {
    inner: PartInner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum PartInner {
    Text(String),
    FunctionCall {
        name: String,
        args: serde_json::Value,
        id: Option<String>,
    },
    FunctionResponse {
        name: String,
        response: serde_json::Value,
        id: Option<String>,
    },
}

#[pymethods]
impl PyPart {
    /// Create a text part
    #[staticmethod]
    fn text(content: String) -> Self {
        Self {
            inner: PartInner::Text(content),
        }
    }

    /// Create a function call part
    #[staticmethod]
    #[pyo3(signature = (name, args, id=None))]
    fn function_call(name: String, args: &Bound<'_, PyAny>, id: Option<String>) -> PyResult<Self> {
        let args_json: serde_json::Value = pythonize::depythonize(args)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            inner: PartInner::FunctionCall {
                name,
                args: args_json,
                id,
            },
        })
    }

    /// Check if this is a text part
    fn is_text(&self) -> bool {
        matches!(self.inner, PartInner::Text(_))
    }

    /// Check if this is a function call
    fn is_function_call(&self) -> bool {
        matches!(self.inner, PartInner::FunctionCall { .. })
    }

    /// Get text content (returns None if not a text part)
    fn get_text(&self) -> Option<String> {
        match &self.inner {
            PartInner::Text(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Get function call name (returns None if not a function call)
    fn get_function_name(&self) -> Option<String> {
        match &self.inner {
            PartInner::FunctionCall { name, .. } => Some(name.clone()),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            PartInner::Text(s) => format!("Part.text('{}')", s),
            PartInner::FunctionCall { name, .. } => format!("Part.function_call('{}')", name),
            PartInner::FunctionResponse { name, .. } => {
                format!("Part.function_response('{}')", name)
            }
        }
    }
}

impl From<adk_core::Part> for PyPart {
    fn from(part: adk_core::Part) -> Self {
        match part {
            adk_core::Part::Text { text } => Self {
                inner: PartInner::Text(text),
            },
            adk_core::Part::FunctionCall { name, args, id } => Self {
                inner: PartInner::FunctionCall { name, args, id },
            },
            adk_core::Part::FunctionResponse {
                function_response,
                id,
            } => Self {
                inner: PartInner::FunctionResponse {
                    name: function_response.name,
                    response: function_response.response,
                    id,
                },
            },
            _ => Self {
                inner: PartInner::Text("[unsupported part type]".to_string()),
            },
        }
    }
}

impl From<PyPart> for adk_core::Part {
    fn from(part: PyPart) -> Self {
        match part.inner {
            PartInner::Text(text) => adk_core::Part::Text { text },
            PartInner::FunctionCall { name, args, id } => {
                adk_core::Part::FunctionCall { name, args, id }
            }
            PartInner::FunctionResponse { name, response, id } => {
                adk_core::Part::FunctionResponse {
                    function_response: adk_core::FunctionResponseData { name, response },
                    id,
                }
            }
        }
    }
}

/// Content represents a message in the conversation
#[pyclass(name = "Content")]
#[derive(Clone, Debug)]
pub struct PyContent {
    #[pyo3(get, set)]
    pub role: String,
    parts: Vec<PyPart>,
}

#[pymethods]
impl PyContent {
    #[new]
    #[pyo3(signature = (role, parts=None))]
    fn new(role: String, parts: Option<Vec<PyPart>>) -> Self {
        Self {
            role,
            parts: parts.unwrap_or_default(),
        }
    }

    /// Create user content with text
    #[staticmethod]
    fn user(text: String) -> Self {
        Self {
            role: "user".to_string(),
            parts: vec![PyPart::text(text)],
        }
    }

    /// Create model content with text
    #[staticmethod]
    fn model(text: String) -> Self {
        Self {
            role: "model".to_string(),
            parts: vec![PyPart::text(text)],
        }
    }

    /// Get all parts
    #[getter]
    fn parts(&self) -> Vec<PyPart> {
        self.parts.clone()
    }

    /// Add a part to this content
    fn add_part(&mut self, part: PyPart) {
        self.parts.push(part);
    }

    /// Get text from all text parts combined
    fn get_text(&self) -> String {
        self.parts
            .iter()
            .filter_map(|p| p.get_text())
            .collect::<Vec<_>>()
            .join("")
    }

    fn __repr__(&self) -> String {
        format!("Content(role='{}', parts={})", self.role, self.parts.len())
    }
}

impl From<adk_core::Content> for PyContent {
    fn from(content: adk_core::Content) -> Self {
        Self {
            role: content.role,
            parts: content.parts.into_iter().map(PyPart::from).collect(),
        }
    }
}

impl From<PyContent> for adk_core::Content {
    fn from(content: PyContent) -> Self {
        Self {
            role: content.role,
            parts: content
                .parts
                .into_iter()
                .map(adk_core::Part::from)
                .collect(),
        }
    }
}

/// An event emitted during agent execution
#[pyclass(name = "Event")]
#[derive(Clone, Debug)]
pub struct PyEvent {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub invocation_id: String,
    #[pyo3(get)]
    pub author: String,
    content: Option<PyContent>,
    #[pyo3(get)]
    pub partial: bool,
    #[pyo3(get)]
    pub turn_complete: bool,
    state_delta: HashMap<String, serde_json::Value>,
}

#[pymethods]
impl PyEvent {
    /// Get the content if present
    #[getter]
    fn content(&self) -> Option<PyContent> {
        self.content.clone()
    }

    /// Get text from content (convenience method)
    fn get_text(&self) -> Option<String> {
        self.content.as_ref().map(|c| c.get_text())
    }

    /// Get state changes from this event
    fn get_state_delta(&self, py: Python<'_>) -> PyResult<PyObject> {
        let bound = pythonize::pythonize(py, &self.state_delta)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(bound.into())
    }

    /// Check if this event has a transfer action
    #[getter]
    fn transfer_to_agent(&self) -> Option<String> {
        None
    }

    fn __repr__(&self) -> String {
        format!(
            "Event(id='{}', author='{}', partial={}, turn_complete={})",
            self.id, self.author, self.partial, self.turn_complete
        )
    }
}

impl From<adk_core::Event> for PyEvent {
    fn from(event: adk_core::Event) -> Self {
        Self {
            id: event.id,
            invocation_id: event.invocation_id,
            author: event.author,
            content: event.llm_response.content.map(PyContent::from),
            partial: event.llm_response.partial,
            turn_complete: event.llm_response.turn_complete,
            state_delta: event.actions.state_delta,
        }
    }
}
