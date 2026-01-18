# adk-rust-python

> **WARNING: UNDER CONSTRUCTION - UNTESTED SOFTWARE**
>
> This project is in active development and has not been thoroughly tested.
> APIs may change without notice. Use at your own risk.
> Not recommended for production use.

Python bindings for [ADK-Rust](https://github.com/zavora-ai/adk-rust) - Build AI agents in Python powered by a high-performance Rust runtime.

**Note:** This package (`adk_rust`) is distinct from Google's ADK.

## Status

This project provides Python bindings for the ADK-Rust framework. Current implementation status:

| Feature | Status |
|---------|--------|
| Core Types (Content, Part, Event) | Implemented |
| LLM Providers (Gemini, OpenAI, Anthropic, etc.) | Implemented |
| LlmAgent with callbacks | Implemented |
| CustomAgent with Python handlers | Implemented |
| Workflow Agents (Sequential, Parallel, Loop) | Implemented |
| Conditional Routing (rule-based & LLM-powered) | Implemented |
| Tools (FunctionTool, AgentTool) | Implemented |
| Guardrails (ContentFilter, PiiRedactor) | Implemented |
| Memory Service | Implemented |
| Artifact Service | Implemented |
| MCP Integration (McpToolset) | Implemented |
| Event Streaming | Not yet implemented |

See `docs/plans/python-bindings-plan.md` for the full implementation roadmap.

## Features

- **High Performance**: Rust-powered execution with Python ergonomics
- **Async Native**: Built for async/await from the ground up
- **Type Safe**: Full type hints and IDE support via `.pyi` stubs
- **Multi-Provider**: Support for Gemini, OpenAI, Anthropic, DeepSeek, Groq, Ollama
- **Extensible**: Define custom tools and agents in Python

## Installation

### From Source

Requires Rust 1.85+ and Python 3.9+.

```bash
# Clone the repository
git clone <repo>
cd adk-rust-python

# Install with uv (recommended)
uv pip install -e ".[dev]"
uv run maturin develop

# Or with pip/maturin directly
pip install maturin
maturin develop
```

## Quick Start

```python
import asyncio
from adk_rust import LlmAgent, GeminiModel, Runner, InMemorySessionService

async def main():
    # Create a model
    model = GeminiModel("your-api-key", "gemini-2.5-flash")
    
    # Build an agent
    agent = (LlmAgent.builder("assistant")
        .model(model)
        .instruction("You are a helpful assistant.")
        .build())
    
    # Run the agent
    session = InMemorySessionService()
    runner = Runner("my_app", agent, session)
    response = await runner.run_simple("user1", "session1", "Hello!")
    print(response)

asyncio.run(main())
```

## Adding Tools

Define tools as Python functions:

```python
from adk_rust import FunctionTool, LlmAgent

def get_weather(ctx, args):
    city = args.get("city", "Unknown")
    return {"city": city, "temperature": 72}

weather_tool = FunctionTool(
    "get_weather",
    "Get current weather for a city",
    get_weather
)

agent = (LlmAgent.builder("assistant")
    .model(model)
    .tool(weather_tool)
    .build())
```

## Custom Agents

Create agents with Python handler functions:

```python
from adk_rust import CustomAgent

async def my_handler(ctx):
    user_text = ctx.user_content.get_text()
    return f"You said: {user_text}"

agent = (CustomAgent.builder("echo")
    .handler(my_handler)
    .description("Echoes user input")
    .build())
```

## Guardrails

Add content filtering and PII redaction:

```python
from adk_rust import ContentFilter, PiiRedactor, GuardrailSet, run_guardrails, Content

# Create guardrails
guardrails = (GuardrailSet()
    .with_content_filter(ContentFilter.harmful_content())
    .with_pii_redactor(PiiRedactor()))

# Run on content
content = Content.user("My email is test@example.com")
result = await run_guardrails(guardrails, content)

if result.passed:
    print("Content is safe")
    if result.transformed_content:
        print(f"Redacted: {result.transformed_content.get_text()}")
```

## Development

```bash
# Build and install in development mode
uv run maturin develop

# Run tests
uv run pytest tests/ -v

# Type checking
uv run mypy python/

# Linting
uv run ruff check .
```

## Architecture

```
Python Application
       |
+------v---------+
|  adk_rust (Py) |  <- This package
+------+---------+
       |
+------v------+
|    PyO3     |  <- Rust/Python bridge  
+------+------+
       |
+------v------+
|  ADK-Rust   |  <- github.com/zavora-ai/adk-rust
+-------------+
```

## License

Apache-2.0
