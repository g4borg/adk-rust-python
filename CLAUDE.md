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
│   │   ├── custom.rs           # CustomAgent (stub)
│   │   ├── workflow.rs         # SequentialAgent, ParallelAgent, LoopAgent
│   │   └── CLAUDE.md           # Agent module docs
│   ├── model/                  # LLM providers
│   │   ├── mod.rs              # All model implementations
│   │   └── CLAUDE.md           # Model module docs
│   ├── tool/                   # Tool system
│   │   ├── mod.rs              # Module exports
│   │   ├── function.rs         # FunctionTool, BasicToolset
│   │   ├── builtin.rs          # ExitLoopTool, LoadArtifactsTool, GoogleSearchTool
│   │   └── CLAUDE.md           # Tool module docs
│   ├── session/                # Session management
│   │   ├── mod.rs              # Session, State, RunConfig
│   │   └── CLAUDE.md           # Session module docs
│   ├── runner/                 # Agent execution
│   │   ├── mod.rs              # Runner, run_agent()
│   │   └── CLAUDE.md           # Runner module docs
│   ├── context.rs              # Context, ToolContext
│   ├── types.rs                # Content, Part, Event
│   └── error.rs                # AdkError
├── python/adk_rust/            # Python package
│   ├── __init__.py             # Public exports
│   ├── __init__.pyi            # Type stubs (keep in sync with Rust!)
│   └── py.typed                # PEP 561 marker
├── tests/                      # pytest tests
├── Cargo.toml                  # Rust dependencies
└── pyproject.toml              # Python/maturin config
```

## Module Documentation

Each `src/*/CLAUDE.md` file contains:
- What's exposed to Python
- What's missing vs adk-rust
- Implementation patterns
- How to add new features

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
- `adk-rust`, `adk-core`, `adk-agent`, `adk-model`, `adk-tool`, `adk-runner`, `adk-session`
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

### Debugging build issues

```bash
# Clean rebuild
cargo clean
maturin develop

# Check Rust compilation separately
cargo check
```

## What's Missing vs adk-rust

See the gap analysis in `C:\Users\g4b\.claude\plans\nested-swinging-scroll.md` or individual module CLAUDE.md files.

**High Priority:**
- CustomAgent (functional implementation)
- Callbacks (before/after model/tool/agent)
- Guardrails (content filtering, PII redaction)

**Medium Priority:**
- Artifact system (binary storage)
- Memory system (semantic search)
- AgentTool (agents as tools)
- ConditionalAgent, MCP integration

## Local Development Notes

See `CLAUDE.local.md` for machine-specific configuration (not committed to git).
