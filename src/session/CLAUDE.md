# Session Module

Python bindings for session and state management.

## Structure

- `mod.rs` - All session-related types

## Exposed Classes

| Python Class | Purpose | Status |
|--------------|---------|--------|
| `InMemorySessionService` | In-memory session storage | ✅ Complete |
| `State` | Key-value state store | ✅ Complete |
| `RunConfig` | Agent execution config | ✅ Complete |
| `StreamingMode` | Streaming behavior enum | ✅ Complete |
| `CreateSessionRequest` | Create session request | ✅ Complete |
| `GetSessionRequest` | Get session request | ✅ Complete |

## State Management

`PyState` is a standalone Python-side state container:

```rust
pub struct PyState {
    data: HashMap<String, serde_json::Value>,
}
```

Uses `pythonize`/`depythonize` for Python ↔ JSON conversion.

### State Prefixes (from adk-core)

The Rust side uses prefixes for state organization:
- `user:` - User preferences (persists across sessions)
- `app:` - Application state
- `temp:` - Temporary data (cleared between runs)

## StreamingMode Enum

PyO3 enum with int representation:

```rust
#[pyclass(name = "StreamingMode", eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PyStreamingMode {
    None = 0,
    SSE = 1,
    Bidi = 2,
}
```

Used in `RunConfig` and converts to `adk_core::StreamingMode`.

## Missing from adk-rust

### Session Service Methods

```python
# Missing:
session_service.list(app_name, user_id)  # List sessions
session_service.delete(app_name, user_id, session_id)  # Delete session
```

### ListSessionRequest, DeleteSessionRequest

Request types for list/delete operations.

### Session Object

Direct access to session data:
```python
session = await session_service.get(request)
session.id
session.state
session.conversation_history
```

## Future: Artifact Service

Binary data storage (images, files):
```python
artifact_service = InMemoryArtifactService()
await artifact_service.save(request)
data = await artifact_service.load(request)
```

## Future: Memory Service

Semantic search/RAG:
```python
memory_service = InMemoryMemoryService()
results = await memory_service.search(query)
```
