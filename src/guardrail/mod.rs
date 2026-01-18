//! Python bindings for guardrails (content filtering, PII redaction)

use adk_guardrail::{
    ContentFilter, ContentFilterConfig, GuardrailExecutor, GuardrailSet, PiiRedactor, PiiType,
    Severity,
};
use pyo3::prelude::*;
use std::sync::Arc;

use crate::types::PyContent;

/// Severity level for guardrail failures
#[pyclass(name = "Severity", eq)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl From<PySeverity> for Severity {
    fn from(s: PySeverity) -> Self {
        match s {
            PySeverity::Low => Severity::Low,
            PySeverity::Medium => Severity::Medium,
            PySeverity::High => Severity::High,
            PySeverity::Critical => Severity::Critical,
        }
    }
}

impl From<Severity> for PySeverity {
    fn from(s: Severity) -> Self {
        match s {
            Severity::Low => PySeverity::Low,
            Severity::Medium => PySeverity::Medium,
            Severity::High => PySeverity::High,
            Severity::Critical => PySeverity::Critical,
        }
    }
}

/// PII types for redaction
#[pyclass(name = "PiiType", eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyPiiType {
    Email,
    Phone,
    Ssn,
    CreditCard,
    IpAddress,
}

impl From<PyPiiType> for PiiType {
    fn from(t: PyPiiType) -> Self {
        match t {
            PyPiiType::Email => PiiType::Email,
            PyPiiType::Phone => PiiType::Phone,
            PyPiiType::Ssn => PiiType::Ssn,
            PyPiiType::CreditCard => PiiType::CreditCard,
            PyPiiType::IpAddress => PiiType::IpAddress,
        }
    }
}

/// Content filter guardrail for blocking harmful or off-topic content
#[pyclass(name = "ContentFilter")]
#[derive(Clone)]
pub struct PyContentFilter {
    pub(crate) inner: Arc<ContentFilter>,
}

#[pymethods]
impl PyContentFilter {
    /// Create a filter that blocks common harmful content patterns
    #[staticmethod]
    fn harmful_content() -> Self {
        Self {
            inner: Arc::new(ContentFilter::harmful_content()),
        }
    }

    /// Create a filter that ensures content is on-topic
    #[staticmethod]
    #[pyo3(signature = (topic, keywords))]
    fn on_topic(topic: String, keywords: Vec<String>) -> Self {
        Self {
            inner: Arc::new(ContentFilter::on_topic(topic, keywords)),
        }
    }

    /// Create a filter with maximum length
    #[staticmethod]
    fn max_length(max: usize) -> Self {
        Self {
            inner: Arc::new(ContentFilter::max_length(max)),
        }
    }

    /// Create a filter with blocked keywords
    #[staticmethod]
    fn blocked_keywords(keywords: Vec<String>) -> Self {
        Self {
            inner: Arc::new(ContentFilter::blocked_keywords(keywords)),
        }
    }

    /// Create a custom content filter with full configuration
    #[staticmethod]
    #[pyo3(signature = (name, blocked_keywords=None, required_topics=None, max_length=None, min_length=None, severity=None))]
    fn custom(
        name: String,
        blocked_keywords: Option<Vec<String>>,
        required_topics: Option<Vec<String>>,
        max_length: Option<usize>,
        min_length: Option<usize>,
        severity: Option<PySeverity>,
    ) -> Self {
        let config = ContentFilterConfig {
            blocked_keywords: blocked_keywords.unwrap_or_default(),
            required_topics: required_topics.unwrap_or_default(),
            max_length,
            min_length,
            severity: severity.map(Into::into).unwrap_or(Severity::High),
        };
        Self {
            inner: Arc::new(ContentFilter::new(name, config)),
        }
    }
}

/// PII detection and redaction guardrail
#[pyclass(name = "PiiRedactor")]
#[derive(Clone)]
pub struct PyPiiRedactor {
    pub(crate) inner: Arc<PiiRedactor>,
}

#[pymethods]
impl PyPiiRedactor {
    /// Create a new PII redactor with all PII types enabled (Email, Phone, SSN, CreditCard)
    #[new]
    fn new() -> Self {
        Self {
            inner: Arc::new(PiiRedactor::new()),
        }
    }

    /// Create a PII redactor with specific types
    #[staticmethod]
    fn with_types(types: Vec<PyPiiType>) -> Self {
        let pii_types: Vec<PiiType> = types.into_iter().map(Into::into).collect();
        Self {
            inner: Arc::new(PiiRedactor::with_types(&pii_types)),
        }
    }

    /// Redact PII from text, returns (redacted_text, found_types)
    fn redact(&self, text: &str) -> (String, Vec<String>) {
        let (redacted, types) = self.inner.redact(text);
        let type_names: Vec<String> = types.iter().map(|t| format!("{:?}", t)).collect();
        (redacted, type_names)
    }
}

/// A set of guardrails to run together
#[pyclass(name = "GuardrailSet")]
#[derive(Clone)]
pub struct PyGuardrailSet {
    pub(crate) content_filters: Vec<Arc<ContentFilter>>,
    pub(crate) pii_redactors: Vec<Arc<PiiRedactor>>,
}

#[pymethods]
impl PyGuardrailSet {
    #[new]
    fn new() -> Self {
        Self {
            content_filters: Vec::new(),
            pii_redactors: Vec::new(),
        }
    }

    /// Add a content filter to this set
    fn with_content_filter(&self, filter: PyContentFilter) -> Self {
        let mut new_set = self.clone();
        new_set.content_filters.push(filter.inner);
        new_set
    }

    /// Add a PII redactor to this set
    fn with_pii_redactor(&self, redactor: PyPiiRedactor) -> Self {
        let mut new_set = self.clone();
        new_set.pii_redactors.push(redactor.inner);
        new_set
    }

    /// Check if this set is empty
    fn is_empty(&self) -> bool {
        self.content_filters.is_empty() && self.pii_redactors.is_empty()
    }
}

impl PyGuardrailSet {
    /// Convert to adk-guardrail GuardrailSet
    pub fn to_guardrail_set(&self) -> GuardrailSet {
        let mut set = GuardrailSet::new();
        for filter in &self.content_filters {
            set = set.with_arc(filter.clone());
        }
        for redactor in &self.pii_redactors {
            set = set.with_arc(redactor.clone());
        }
        set
    }
}

/// Result of running guardrails
#[pyclass(name = "GuardrailResult")]
pub struct PyGuardrailResult {
    #[pyo3(get)]
    pub passed: bool,
    #[pyo3(get)]
    pub transformed_content: Option<PyContent>,
    #[pyo3(get)]
    pub failures: Vec<PyGuardrailFailure>,
}

#[pymethods]
impl PyGuardrailResult {
    fn __repr__(&self) -> String {
        format!(
            "GuardrailResult(passed={}, failures={})",
            self.passed,
            self.failures.len()
        )
    }
}

/// A single guardrail failure
#[pyclass(name = "GuardrailFailure")]
#[derive(Clone)]
pub struct PyGuardrailFailure {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub reason: String,
    #[pyo3(get)]
    pub severity: PySeverity,
}

#[pymethods]
impl PyGuardrailFailure {
    fn __repr__(&self) -> String {
        format!(
            "GuardrailFailure(name='{}', reason='{}', severity={:?})",
            self.name, self.reason, self.severity
        )
    }
}

/// Run guardrails on content
#[pyfunction]
#[pyo3(signature = (guardrails, content))]
pub fn run_guardrails<'py>(
    py: Python<'py>,
    guardrails: PyGuardrailSet,
    content: PyContent,
) -> PyResult<Bound<'py, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let set = guardrails.to_guardrail_set();
        let core_content = content.into();

        let result = GuardrailExecutor::run(&set, &core_content)
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let failures: Vec<PyGuardrailFailure> = result
            .failures
            .into_iter()
            .map(|(name, reason, severity)| PyGuardrailFailure {
                name,
                reason,
                severity: severity.into(),
            })
            .collect();

        Ok(PyGuardrailResult {
            passed: result.passed,
            transformed_content: result.transformed_content.map(PyContent::from),
            failures,
        })
    })
}
