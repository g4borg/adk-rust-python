//! Python bindings for artifact services

use adk_artifact::{
    ArtifactService, DeleteRequest, InMemoryArtifactService, ListRequest, LoadRequest, SaveRequest,
    VersionsRequest,
};
use adk_core::Part;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::sync::Arc;

use crate::types::PyPart;

/// In-memory artifact service for binary data storage
#[pyclass(name = "InMemoryArtifactService")]
pub struct PyInMemoryArtifactService {
    inner: Arc<InMemoryArtifactService>,
}

#[pymethods]
impl PyInMemoryArtifactService {
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(InMemoryArtifactService::new()),
        }
    }

    /// Save an artifact (bytes or text)
    ///
    /// Args:
    ///     app_name: Application name
    ///     user_id: User ID
    ///     session_id: Session ID
    ///     file_name: Artifact name (prefix with "user:" for user-scoped)
    ///     data: Binary data (bytes) or text (str)
    ///     mime_type: Optional MIME type (defaults to application/octet-stream for bytes)
    ///     version: Optional version number (auto-increments if not specified)
    ///
    /// Returns:
    ///     Version number of saved artifact
    #[pyo3(signature = (app_name, user_id, session_id, file_name, data, mime_type=None, version=None))]
    fn save<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        session_id: String,
        file_name: String,
        data: Bound<'py, PyAny>,
        mime_type: Option<String>,
        version: Option<i64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        // Convert data to Part
        let part: Part = if let Ok(bytes) = data.downcast::<PyBytes>() {
            let bytes_vec = bytes.as_bytes().to_vec();
            let mime = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
            Part::InlineData {
                mime_type: mime,
                data: bytes_vec,
            }
        } else if let Ok(text) = data.extract::<String>() {
            Part::Text { text }
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "data must be bytes or str",
            ));
        };

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let request = SaveRequest {
                app_name,
                user_id,
                session_id,
                file_name,
                part,
                version,
            };

            let response = inner
                .save(request)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(response.version)
        })
    }

    /// Load an artifact
    ///
    /// Args:
    ///     app_name: Application name
    ///     user_id: User ID
    ///     session_id: Session ID
    ///     file_name: Artifact name
    ///     version: Optional version (loads latest if not specified)
    ///
    /// Returns:
    ///     Part containing the artifact data
    #[pyo3(signature = (app_name, user_id, session_id, file_name, version=None))]
    fn load<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        session_id: String,
        file_name: String,
        version: Option<i64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let request = LoadRequest {
                app_name,
                user_id,
                session_id,
                file_name,
                version,
            };

            let response = inner
                .load(request)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(PyPart::from(response.part))
        })
    }

    /// Delete an artifact
    ///
    /// Args:
    ///     app_name: Application name
    ///     user_id: User ID
    ///     session_id: Session ID
    ///     file_name: Artifact name
    ///     version: Optional version (deletes all versions if not specified)
    #[pyo3(signature = (app_name, user_id, session_id, file_name, version=None))]
    fn delete<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        session_id: String,
        file_name: String,
        version: Option<i64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let request = DeleteRequest {
                app_name,
                user_id,
                session_id,
                file_name,
                version,
            };

            inner
                .delete(request)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(())
        })
    }

    /// List all artifact names in a session
    ///
    /// Args:
    ///     app_name: Application name
    ///     user_id: User ID
    ///     session_id: Session ID
    ///
    /// Returns:
    ///     List of artifact file names
    #[pyo3(signature = (app_name, user_id, session_id))]
    fn list<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        session_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let request = ListRequest {
                app_name,
                user_id,
                session_id,
            };

            let response = inner
                .list(request)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(response.file_names)
        })
    }

    /// Get all versions of an artifact
    ///
    /// Args:
    ///     app_name: Application name
    ///     user_id: User ID
    ///     session_id: Session ID
    ///     file_name: Artifact name
    ///
    /// Returns:
    ///     List of version numbers (descending order)
    #[pyo3(signature = (app_name, user_id, session_id, file_name))]
    fn versions<'py>(
        &self,
        py: Python<'py>,
        app_name: String,
        user_id: String,
        session_id: String,
        file_name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let request = VersionsRequest {
                app_name,
                user_id,
                session_id,
                file_name,
            };

            let response = inner
                .versions(request)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(response.versions)
        })
    }
}

impl PyInMemoryArtifactService {
    /// Get the inner Arc for use in Runner
    pub fn inner(&self) -> Arc<InMemoryArtifactService> {
        self.inner.clone()
    }
}
