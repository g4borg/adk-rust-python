//! Python bindings for memory services

use adk_memory::{InMemoryMemoryService, MemoryEntry, MemoryService, SearchRequest};
use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use std::sync::Arc;

use crate::types::PyContent;

/// A memory entry with content and metadata
#[pyclass(name = "MemoryEntry")]
#[derive(Clone)]
pub struct PyMemoryEntry {
    #[pyo3(get)]
    pub content: PyContent,
    #[pyo3(get)]
    pub author: String,
    #[pyo3(get)]
    pub timestamp: String,
}

#[pymethods]
impl PyMemoryEntry {
    #[new]
    #[pyo3(signature = (content, author, timestamp=None))]
    fn new(content: PyContent, author: String, timestamp: Option<String>) -> Self {
        let ts = timestamp.unwrap_or_else(|| Utc::now().to_rfc3339());
        Self {
            content,
            author,
            timestamp: ts,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "MemoryEntry(author='{}', timestamp='{}')",
            self.author, self.timestamp
        )
    }
}

impl From<MemoryEntry> for PyMemoryEntry {
    fn from(entry: MemoryEntry) -> Self {
        Self {
            content: PyContent::from(entry.content),
            author: entry.author,
            timestamp: entry.timestamp.to_rfc3339(),
        }
    }
}

impl TryFrom<PyMemoryEntry> for MemoryEntry {
    type Error = PyErr;

    fn try_from(entry: PyMemoryEntry) -> Result<Self, Self::Error> {
        let timestamp: DateTime<Utc> = DateTime::parse_from_rfc3339(&entry.timestamp)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
            .with_timezone(&Utc);

        Ok(Self {
            content: entry.content.into(),
            author: entry.author,
            timestamp,
        })
    }
}

/// In-memory memory service for semantic search
#[pyclass(name = "InMemoryMemoryService")]
pub struct PyInMemoryMemoryService {
    inner: Arc<InMemoryMemoryService>,
}

#[pymethods]
impl PyInMemoryMemoryService {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(InMemoryMemoryService::new()),
        }
    }

    /// Add session memories
    #[pyo3(signature = (app_name, user_id, session_id, entries))]
    fn add_session<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        session_id: String,
        entries: Vec<PyMemoryEntry>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let rust_entries: Vec<MemoryEntry> = entries
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?;

            inner
                .add_session(&app_name, &user_id, &session_id, rust_entries)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(())
        })
    }

    /// Search for memories matching a query
    #[pyo3(signature = (app_name, user_id, query))]
    fn search<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        query: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let request = SearchRequest {
                query,
                user_id,
                app_name,
            };

            let response = inner
                .search(request)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            let memories: Vec<PyMemoryEntry> = response
                .memories
                .into_iter()
                .map(PyMemoryEntry::from)
                .collect();

            Ok(memories)
        })
    }
}

impl PyInMemoryMemoryService {
    /// Get the inner Arc for use in Runner
    pub fn inner(&self) -> Arc<InMemoryMemoryService> {
        self.inner.clone()
    }
}
