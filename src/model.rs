//! Model wrappers for Python - All LLM providers

use pyo3::prelude::*;
use std::sync::Arc;

/// Google Gemini model wrapper
#[pyclass(name = "GeminiModel")]
#[derive(Clone)]
pub struct PyGeminiModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyGeminiModel {
    #[new]
    #[pyo3(signature = (api_key, model="gemini-2.5-flash"))]
    fn new(api_key: String, model: &str) -> PyResult<Self> {
        let gemini = adk_model::GeminiModel::new(&api_key, model)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(gemini) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("GeminiModel(name='{}')", self.name()) }
}

/// OpenAI model wrapper
#[pyclass(name = "OpenAIModel")]
#[derive(Clone)]
pub struct PyOpenAIModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyOpenAIModel {
    #[new]
    #[pyo3(signature = (api_key, model="gpt-4o"))]
    fn new(api_key: String, model: &str) -> PyResult<Self> {
        let config = adk_model::OpenAIConfig::new(&api_key, model);
        let client = adk_model::OpenAIClient::new(config)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn compatible(api_key: String, base_url: String, model: String) -> PyResult<Self> {
        let client = adk_model::OpenAIClient::compatible(&api_key, &base_url, &model)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("OpenAIModel(name='{}')", self.name()) }
}

/// Anthropic Claude model wrapper
#[pyclass(name = "AnthropicModel")]
#[derive(Clone)]
pub struct PyAnthropicModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyAnthropicModel {
    #[new]
    #[pyo3(signature = (api_key, model="claude-sonnet-4-20250514"))]
    fn new(api_key: String, model: &str) -> PyResult<Self> {
        let config = adk_model::anthropic::AnthropicConfig::new(&api_key, model);
        let client = adk_model::AnthropicClient::new(config)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn from_api_key(api_key: String) -> PyResult<Self> {
        let client = adk_model::AnthropicClient::from_api_key(&api_key)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("AnthropicModel(name='{}')", self.name()) }
}

/// DeepSeek model wrapper
#[pyclass(name = "DeepSeekModel")]
#[derive(Clone)]
pub struct PyDeepSeekModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyDeepSeekModel {
    #[new]
    fn new(api_key: String, model: String) -> PyResult<Self> {
        let config = adk_model::DeepSeekConfig::new(&api_key, &model);
        let client = adk_model::DeepSeekClient::new(config)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn chat(api_key: String) -> PyResult<Self> {
        let client = adk_model::DeepSeekClient::chat(&api_key)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn reasoner(api_key: String) -> PyResult<Self> {
        let client = adk_model::DeepSeekClient::reasoner(&api_key)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("DeepSeekModel(name='{}')", self.name()) }
}

/// Groq model wrapper (fast inference)
#[pyclass(name = "GroqModel")]
#[derive(Clone)]
pub struct PyGroqModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyGroqModel {
    #[new]
    fn new(api_key: String, model: String) -> PyResult<Self> {
        let config = adk_model::GroqConfig::new(&api_key, &model);
        let client = adk_model::GroqClient::new(config)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn llama70b(api_key: String) -> PyResult<Self> {
        let client = adk_model::GroqClient::llama70b(&api_key)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn llama8b(api_key: String) -> PyResult<Self> {
        let client = adk_model::GroqClient::llama8b(&api_key)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[staticmethod]
    fn mixtral(api_key: String) -> PyResult<Self> {
        let client = adk_model::GroqClient::mixtral(&api_key)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(client) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("GroqModel(name='{}')", self.name()) }
}

/// Ollama model wrapper (local inference)
#[pyclass(name = "OllamaModel")]
#[derive(Clone)]
pub struct PyOllamaModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyOllamaModel {
    #[new]
    fn new(model: String) -> PyResult<Self> {
        let ollama = adk_model::OllamaModel::from_model(&model)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(ollama) })
    }

    #[staticmethod]
    fn with_host(host: String, model: String) -> PyResult<Self> {
        let config = adk_model::OllamaConfig::with_host(&host, &model);
        let ollama = adk_model::OllamaModel::new(config)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner: Arc::new(ollama) })
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("OllamaModel(name='{}')", self.name()) }
}

/// Mock LLM for testing
#[pyclass(name = "MockLlm")]
#[derive(Clone)]
pub struct PyMockLlm {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}

#[pymethods]
impl PyMockLlm {
    #[new]
    #[pyo3(signature = (name, response_text="Mock response"))]
    fn new(name: String, response_text: &str) -> Self {
        let response = adk_core::LlmResponse::new(
            adk_core::Content::new("model").with_text(response_text)
        );
        let mock = adk_model::MockLlm::new(&name).with_response(response);
        Self { inner: Arc::new(mock) }
    }

    #[getter]
    fn name(&self) -> String { self.inner.name().to_string() }

    fn __repr__(&self) -> String { format!("MockLlm(name='{}')", self.name()) }
}
