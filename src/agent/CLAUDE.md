# Agent Module

Python bindings for ADK agent types.

## Structure

- `mod.rs` - Module exports
- `llm.rs` - `LlmAgent` and `LlmAgentBuilder`
- `custom.rs` - `CustomAgent` (stub - to be implemented)
- `workflow.rs` - `SequentialAgent`, `ParallelAgent`, `LoopAgent`

## Exposed Classes

| Python Class | Rust Source | Status |
|--------------|-------------|--------|
| `LlmAgent` | `llm.rs` | ✅ Complete |
| `LlmAgentBuilder` | `llm.rs` | ✅ Complete |
| `CustomAgent` | `custom.rs` | ⚠️ Stub only |
| `SequentialAgent` | `workflow.rs` | ✅ Complete |
| `ParallelAgent` | `workflow.rs` | ✅ Complete |
| `LoopAgent` | `workflow.rs` | ✅ Complete |

## Missing from adk-rust

These agent types exist in upstream but are not yet bound:

- **`CustomAgent` (functional)** - Allow Python functions as agent logic
- **`ConditionalAgent`** - Branch based on conditions
- **`LlmConditionalAgent`** - LLM-powered branching
- **Callbacks** - `before_model_callback`, `after_model_callback`, etc.
- **Guardrails** - `input_guardrails`, `output_guardrails`

## LlmAgentBuilder Pattern

The builder uses PyO3's `PyRefMut` for method chaining:

```rust
fn description(mut slf: PyRefMut<'_, Self>, desc: String) -> PyRefMut<'_, Self> {
    slf.description = Some(desc);
    slf
}
```

This allows Python code like:
```python
agent = (LlmAgent.builder("my_agent")
    .description("Does things")
    .instruction("Be helpful")
    .model(model)
    .tool(my_tool)
    .build())
```

## Adding New Agent Types

1. Create new file in `src/agent/` (e.g., `conditional.rs`)
2. Define `#[pyclass]` struct wrapping the Rust agent
3. Implement `#[pymethods]` for Python-callable methods
4. Export from `mod.rs`
5. Register in `lib.rs` via `m.add_class::<PyNewAgent>()?`
6. Add to `python/adk_rust/__init__.py` and `__init__.pyi`
