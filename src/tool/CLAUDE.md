# Tool Module

Python bindings for the ADK tool system.

## Structure

- `mod.rs` - Module exports
- `function.rs` - `FunctionTool`, `BasicToolset`
- `builtin.rs` - `ExitLoopTool`, `LoadArtifactsTool`, `GoogleSearchTool`

## Exposed Classes

| Python Class | Purpose | Status |
|--------------|---------|--------|
| `FunctionTool` | Wrap Python functions | ✅ Complete |
| `BasicToolset` | Group tools together | ✅ Complete |
| `ExitLoopTool` | Exit loop agents | ✅ Complete |
| `LoadArtifactsTool` | Load artifacts | ✅ Complete |
| `GoogleSearchTool` | Google search | ✅ Complete |

## FunctionTool Implementation

The most complex binding - wraps a Python callable as a Rust `Tool`:

```rust
struct PythonTool {
    name: String,
    description: String,
    handler: Py<PyAny>,  // Python callable
    parameters_schema: Option<serde_json::Value>,
}
```

### Execution Flow

1. Tool called by Rust runtime with `ctx` and `args`
2. `tokio::task::spawn_blocking` to avoid blocking async runtime
3. `Python::with_gil` to acquire GIL
4. Convert args to Python via `pythonize`
5. Call Python handler with `(ToolContext, dict)`
6. Convert result back via `depythonize`

### Thread Safety

`PythonTool` manually implements `Send + Sync`:

```rust
unsafe impl Send for PythonTool {}
unsafe impl Sync for PythonTool {}
```

This is safe because:
- `Py<PyAny>` is thread-safe (reference counted)
- All Python access happens inside `Python::with_gil`

## Missing from adk-rust

### AgentTool (HIGH PRIORITY)
Use agents as tools for composition:
```rust
AgentTool::new(agent, config)
```

### McpToolset (MEDIUM PRIORITY)
Connect to MCP servers for external tools:
```rust
McpToolset::connect("npx", ["-y", "@modelcontextprotocol/server-filesystem"])
```

### Long-Running Tools
`is_long_running()` support for tools that take time.

## Adding Built-in Tools

1. Add struct in `builtin.rs`:
```rust
#[pyclass(name = "MyTool")]
#[derive(Clone)]
pub struct PyMyTool {
    pub(crate) inner: Arc<dyn Tool>,
}
```

2. Implement constructor using adk-tool type:
```rust
#[new]
fn new() -> Self {
    Self { inner: Arc::new(adk_tool::MyTool::new()) }
}
```

3. Add `name`, `description` getters and `__repr__`
4. Export from `mod.rs`
5. Register in `lib.rs`
