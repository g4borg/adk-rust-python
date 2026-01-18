# Agent Module

Python bindings for ADK agent types.

## Structure

- `mod.rs` - Module exports
- `llm.rs` - `LlmAgent` and `LlmAgentBuilder` (with callback support)
- `custom.rs` - `CustomAgent` and `CustomAgentBuilder`
- `conditional.rs` - `ConditionalAgent`, `LlmConditionalAgent`, `LlmConditionalAgentBuilder`
- `workflow.rs` - `SequentialAgent`, `ParallelAgent`, `LoopAgent`

## Exposed Classes

| Python Class | Rust Source | Status |
|--------------|-------------|--------|
| `LlmAgent` | `llm.rs` | Complete |
| `LlmAgentBuilder` | `llm.rs` | Complete (with callbacks) |
| `CustomAgent` | `custom.rs` | Complete |
| `CustomAgentBuilder` | `custom.rs` | Complete |
| `ConditionalAgent` | `conditional.rs` | Complete |
| `LlmConditionalAgent` | `conditional.rs` | Complete |
| `LlmConditionalAgentBuilder` | `conditional.rs` | Complete |
| `SequentialAgent` | `workflow.rs` | Complete |
| `ParallelAgent` | `workflow.rs` | Complete |
| `LoopAgent` | `workflow.rs` | Complete |

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
    .before_model_callback(my_callback)
    .build())
```

## CustomAgent with Python Handlers

CustomAgent accepts Python async functions as handlers:

```python
async def my_handler(ctx: InvocationContext) -> str:
    return f"Processed: {ctx.user_content.get_text()}"

agent = (CustomAgent.builder("my_agent")
    .handler(my_handler)
    .description("Custom logic")
    .build())
```

## ConditionalAgent (Rule-based)

Routes execution based on a Python condition function:

```python
router = ConditionalAgent(
    "router",
    condition=lambda ctx: ctx.state.get("premium"),
    if_agent=premium_agent,
    else_agent=basic_agent
)
```

## LlmConditionalAgent (LLM-powered)

Uses an LLM to classify and route to sub-agents:

```python
router = (LlmConditionalAgent.builder("router", model)
    .instruction("Classify as 'technical', 'billing', or 'general'")
    .route("technical", tech_agent)
    .route("billing", billing_agent)
    .default_route(general_agent)
    .build())
```

## Callbacks

LlmAgentBuilder supports six callback types:

- `before_agent_callback(fn)` - Before agent execution
- `after_agent_callback(fn)` - After agent execution
- `before_model_callback(fn)` - Before LLM call (can skip)
- `after_model_callback(fn)` - After LLM call (can modify)
- `before_tool_callback(fn)` - Before tool execution
- `after_tool_callback(fn)` - After tool execution

## Adding New Agent Types

1. Create new file in `src/agent/` (e.g., `new_agent.rs`)
2. Define `#[pyclass]` struct wrapping the Rust agent
3. Implement `#[pymethods]` for Python-callable methods
4. Export from `mod.rs`
5. Register in `lib.rs` via `m.add_class::<PyNewAgent>()?`
6. Add to `python/adk_rust/__init__.py` and `__init__.pyi`
