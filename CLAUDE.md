# CLAUDE.md - adk-rust-python

Python bindings for ADK-Rust, enabling AI agent development in Python with a high-performance Rust runtime.

## Project Structure

```
adk-rust-python/
├── src/                        # Rust source (PyO3 bindings)
│   ├── lib.rs                  # Module entry, exports all classes
│   ├── agent/                  # Agent types
│   │   ├── mod.rs              # Module exports
│   │   ├── llm.rs              # LlmAgent, LlmAgentBuilder
│   │   ├── custom.rs           # CustomAgent, CustomAgentBuilder
│   │   ├── conditional.rs      # ConditionalAgent, LlmConditionalAgent
│   │   └── workflow.rs         # SequentialAgent, ParallelAgent, LoopAgent
│   ├── model/                  # LLM providers
│   │   └── mod.rs              # All model implementations
│   ├── tool/                   # Tool system
│   │   ├── mod.rs              # Module exports
│   │   ├── function.rs         # FunctionTool, BasicToolset
│   │   ├── agent_tool.rs       # AgentTool (agents as tools)
│   │   ├── mcp.rs              # McpToolset (MCP server integration)
│   │   └── builtin.rs          # ExitLoopTool, LoadArtifactsTool, GoogleSearchTool
│   ├── session/                # Session management
│   │   └── mod.rs              # Session, State, RunConfig, GenerateContentConfig
│   ├── runner/                 # Agent execution
│   │   └── mod.rs              # Runner, run_agent()
│   ├── guardrail/              # Content safety
│   │   └── mod.rs              # ContentFilter, PiiRedactor, GuardrailSet
│   ├── callbacks.rs            # Callback types (LlmRequest, LlmResponse, etc.)
│   ├── context.rs              # Context, ToolContext, InvocationContext, CallbackContext
│   ├── memory.rs               # InMemoryMemoryService, MemoryEntry
│   ├── artifact.rs             # InMemoryArtifactService
│   ├── types.rs                # Content, Part, Event
│   └── error.rs                # AdkError
├── python/adk_rust/            # Python package
│   ├── __init__.py             # Public exports
│   ├── __init__.pyi            # Type stubs (keep in sync with Rust!)
│   └── py.typed                # PEP 561 marker
├── tests/                      # pytest tests
├── docs/plans/                 # Implementation plans
├── Cargo.toml                  # Rust dependencies
└── pyproject.toml              # Python/maturin config
```

## Build System

This project uses **maturin** to build Rust code as a Python extension module via **PyO3**.

### Development Commands

```bash
# Build and install in development mode (rebuilds Rust)
maturin develop

# Run tests
pytest tests/

# Type checking
mypy python/

# Linting
ruff check .

# Build release wheel
maturin build --release
```

Note: use `uv run` if running commands fails.

### Virtual Environment

Uses `uv` for venv management. Dev dependencies are in `[project.optional-dependencies].dev`:

```bash
uv pip install -e ".[dev]"
```

## What's Exposed to Python

**Types:** `Content`, `Part`, `Event`

**Models:** `GeminiModel`, `OpenAIModel`, `AnthropicModel`, `DeepSeekModel`, `GroqModel`, `OllamaModel`, `MockLlm`

**Agents:** `LlmAgent`, `LlmAgentBuilder`, `CustomAgent`, `CustomAgentBuilder`, `SequentialAgent`, `ParallelAgent`, `LoopAgent`, `ConditionalAgent`, `LlmConditionalAgent`, `LlmConditionalAgentBuilder`

**Tools:** `FunctionTool`, `BasicToolset`, `AgentTool`, `ExitLoopTool`, `GoogleSearchTool`, `LoadArtifactsTool`, `McpToolset`

**Session:** `InMemorySessionService`, `State`, `RunConfig`, `StreamingMode`, `CreateSessionRequest`, `GetSessionRequest`, `GenerateContentConfig`

**Runner:** `Runner`, `run_agent()`

**Context:** `Context`, `ToolContext`, `InvocationContext`, `CallbackContext`

**Callbacks:** `LlmRequest`, `LlmResponse`, `BeforeModelResult`

**Guardrails:** `Severity`, `PiiType`, `ContentFilter`, `PiiRedactor`, `GuardrailSet`, `GuardrailResult`, `GuardrailFailure`, `run_guardrails()`

**Memory:** `MemoryEntry`, `InMemoryMemoryService`

**Artifacts:** `InMemoryArtifactService`

**Error:** `AdkError`

## Key Patterns

### PyO3 Bindings

Rust structs are wrapped with `#[pyclass]` and expose methods via `#[pymethods]`:

```rust
#[pyclass]
pub struct MyClass {
    inner: Arc<dyn SomeTrait>,
}

#[pymethods]
impl MyClass {
    #[new]
    #[pyo3(signature = (name, value=None))]
    fn new(name: String, value: Option<i32>) -> Self { ... }
    
    #[getter]
    fn name(&self) -> String { ... }
}
```

### Type Stubs

The `.pyi` file in `python/adk_rust/__init__.pyi` must stay in sync with the Rust bindings. When adding/changing a `#[pyclass]` or `#[pymethods]`, update the stub.

### Async

Uses `pyo3-async-runtimes` with Tokio. Async methods return Python awaitables:

```rust
fn run<'py>(&self, py: Python<'py>, ...) -> PyResult<Bound<'py, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move { ... })
}
```

## Dependencies

Rust ADK modules are sourced from git:
- `adk-rust`, `adk-core`, `adk-agent`, `adk-model`, `adk-tool`, `adk-runner`, `adk-session`, `adk-guardrail`, `adk-memory`, `adk-artifact`
- All from: `https://github.com/zavora-ai/adk-rust`

## Testing

```bash
pytest tests/           # Run all tests
pytest tests/ -v        # Verbose
pytest tests/ -k test_name  # Run specific test
```

Tests use `pytest-asyncio` with `asyncio_mode = "auto"`.

## Common Tasks

### Adding a new Python-exposed class

1. Create/modify the Rust struct in appropriate `src/*/` module with `#[pyclass]`
2. Add `#[pymethods]` for Python-callable methods
3. Export from the module's `mod.rs`
4. Register in `src/lib.rs` via `m.add_class::<ClassName>()?`
5. Add to `python/adk_rust/__init__.py` exports
6. Add type stub in `python/adk_rust/__init__.pyi`
7. Add tests in `tests/`
8. **Update this CLAUDE.md** to reflect the new class in the appropriate section

### Debugging build issues

```bash
# Clean rebuild
cargo clean
maturin develop

# Check Rust compilation separately
cargo check
```

## Future Work

See `docs/plans/python-bindings-plan.md` for the full implementation plan.

**Remaining (Phase 4):**
- Event Streaming (async iteration)

**Future (Phase 5+):**
- Browser integration
- Graph workflows
- Schema serialization

## Documentation Guidelines

**Keep documentation in sync with implementation.** When completing work from `docs/plans/` or making significant changes:
1. Update the relevant `CLAUDE.md` file(s) in the affected folder/topic
2. Update `README.md` if user-facing APIs or usage patterns changed
3. Update this root `CLAUDE.md` if the change affects project structure or exposed classes

Each folder may have its own `CLAUDE.md` with topic-specific guidance - keep those current as you work in that area.

## Local Development Notes

See `CLAUDE.local.md` for machine-specific configuration (not committed to git).
