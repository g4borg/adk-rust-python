# adk-rust-python

Python bindings for [ADK-Rust](https://github.com/zavora-ai/adk-rust) - Build AI agents in Python powered by a high-performance Rust runtime.

**Note:** This package (`adk_rust`) is distinct from Google's ADK.

## Features

- **High Performance**: Rust-powered execution with Python ergonomics
- **Async Native**: Built for async/await from the ground up
- **Type Safe**: Full type hints and IDE support
- **Gemini First**: Optimized for Google Gemini models
- **Extensible**: Define custom tools in Python

## Installation

### From Source

Requires Rust 1.85+ and Python 3.9+.

    # Install maturin
    pip install maturin

    # Build and install in development mode
    maturin develop

    # Or build a wheel
    maturin build --release

## Quick Start

    import asyncio
    from adk_rust import LlmAgent, GeminiModel, Runner

    async def main():
        # Create a model
        model = GeminiModel("your-api-key", "gemini-2.5-flash")
        
        # Build an agent
        agent = LlmAgent.builder("assistant") \
            .model(model) \
            .instruction("You are a helpful assistant.") \
            .build()
        
        # Run the agent
        runner = Runner(agent)
        response = await runner.run_sync("user1", "session1", "Hello!")
        print(response)

    asyncio.run(main())

## Adding Tools

Define tools as Python functions:

    from adk_rust import FunctionTool

    def get_weather(ctx, args):
        city = args.get("city", "Unknown")
        return {"city": city, "temperature": 72}

    weather_tool = FunctionTool(
        "get_weather",
        "Get current weather for a city",
        get_weather
    )

    agent = LlmAgent.builder("assistant") \
        .model(model) \
        .tool(weather_tool) \
        .build()

## Development

    # Clone and setup
    git clone <repo>
    cd adk-rust-python
    pip install maturin pytest pytest-asyncio
    maturin develop
    pytest tests/

## Architecture

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

## License

Apache-2.0
