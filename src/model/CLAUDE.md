# Model Module

Python bindings for LLM providers.

## Structure

- `mod.rs` - All model implementations and `extract_llm()` helper

## Exposed Classes

| Python Class | Provider | Status |
|--------------|----------|--------|
| `GeminiModel` | Google Gemini | Complete |
| `OpenAIModel` | OpenAI GPT | Complete |
| `AnthropicModel` | Anthropic Claude | Complete |
| `DeepSeekModel` | DeepSeek | Complete |
| `GroqModel` | Groq | Complete |
| `OllamaModel` | Ollama (local) | Complete |
| `MockLlm` | Testing | Complete |

## Key Patterns

### All Models Share the Same Inner Type

```rust
pub struct PyGeminiModel {
    pub(crate) inner: Arc<dyn adk_core::Llm>,
}
```

This allows them to be used interchangeably via `extract_llm()`.

### The `extract_llm()` Helper

Extracts `Arc<dyn Llm>` from any Python model object:

```rust
pub fn extract_llm(obj: &Bound<'_, PyAny>) -> PyResult<Arc<dyn adk_core::Llm>> {
    if let Ok(model) = obj.extract::<PyRef<'_, PyGeminiModel>>() {
        return Ok(model.inner.clone());
    }
    // ... try other types
}
```

Used by `LlmAgentBuilder.model()` to accept any model type.

### Static Convenience Constructors

Most models have static methods for common configurations:

```python
# Instead of remembering model names:
model = DeepSeekModel.chat(api_key)      # deepseek-chat
model = DeepSeekModel.reasoner(api_key)  # deepseek-reasoner
model = GroqModel.llama70b(api_key)      # llama-3.3-70b-versatile
```

### OpenAI-Compatible API

`OpenAIModel.compatible()` supports any OpenAI-compatible API:

```python
model = OpenAIModel.compatible(
    api_key="key",
    base_url="https://api.together.xyz/v1",
    model="meta-llama/Llama-3-70b-chat-hf"
)
```

## Missing from adk-rust

- **Azure OpenAI** - `AzureOpenAIClient` (needs separate class or config)

## Adding a New Model Provider

1. Add `#[pyclass]` struct with `inner: Arc<dyn adk_core::Llm>`
2. Implement `#[new]` constructor calling the Rust model
3. Add `#[getter] fn name()` and `fn __repr__()`
4. Add case to `extract_llm()` function
5. Register in `lib.rs`
6. Update Python stubs
