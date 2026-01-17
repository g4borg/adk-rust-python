# Runner Module

Python bindings for agent execution.

## Structure

- `mod.rs` - `Runner` class and `run_agent()` function

## Exposed Classes/Functions

| Python Name | Purpose | Status |
|-------------|---------|--------|
| `Runner` | Full-featured agent executor | ✅ Complete |
| `run_agent()` | Simple one-shot execution | ✅ Complete |

## Runner Class

Executes agents with full configuration:

```python
runner = Runner(
    app_name="my_app",
    agent=my_agent,
    session_service=InMemorySessionService(),
    run_config=RunConfig(streaming_mode=StreamingMode.SSE)
)

# Get all events
events = await runner.run(user_id, session_id, "Hello")

# Get just final text
response = await runner.run_simple(user_id, session_id, "Hello")
```

## Async Execution

Uses `pyo3_async_runtimes::tokio::future_into_py` to convert Rust futures to Python awaitables:

```rust
fn run<'py>(&self, py: Python<'py>, ...) -> PyResult<Bound<'py, PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        // Rust async code here
    })
}
```

## Event Streaming

The runner internally streams events, but currently collects them all before returning:

```rust
let mut events = Vec::new();
while let Some(result) = stream.next().await {
    match result {
        Ok(event) => events.push(PyEvent::from(event)),
        Err(e) => return Err(...),
    }
}
Ok(events)
```

## Missing from adk-rust

### Artifact & Memory Services

Current runner config always sets these to `None`:

```rust
let config = adk_runner::RunnerConfig {
    // ...
    artifact_service: None,  // TODO
    memory_service: None,    // TODO
};
```

### Callbacks

The Rust `Runner` supports callbacks but they're not exposed:

```rust
// In Rust:
runner.with_before_model_callback(|ctx, req| { ... })
runner.with_after_model_callback(|ctx, resp| { ... })
```

### True Streaming

Currently we collect all events. For true streaming we'd need:
- Python async generator
- Or callback-based approach
- Or return an async iterator

### Agent Type Flexibility

Currently only accepts `PyLlmAgent`. Should accept any agent type:
- `SequentialAgent`
- `ParallelAgent`
- `LoopAgent`
- `CustomAgent`

## run_agent() Convenience Function

One-shot execution with sensible defaults:

```python
response = await run_agent(
    agent=my_agent,
    message="Hello",
    user_id="default_user",
    session_id="default_session",
    app_name="adk_app"
)
```

Creates a fresh `InMemorySessionService` for each call - useful for stateless testing.
