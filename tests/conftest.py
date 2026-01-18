"""Shared pytest fixtures for adk_rust tests."""

import pytest
from adk_rust import (
    BasicToolset,
    # Types
    Content,
    ContentFilter,
    FunctionTool,
    # Guardrails
    GuardrailSet,
    InMemoryArtifactService,
    # Memory & Artifacts
    InMemoryMemoryService,
    # Session
    InMemorySessionService,
    # Agents
    LlmAgent,
    # Models
    MockLlm,
    PiiRedactor,
    RunConfig,
    State,
)

# =============================================================================
# Model Fixtures
# =============================================================================


@pytest.fixture
def mock_model():
    """Create a MockLlm that returns a simple response."""
    return MockLlm("test_model", "Mock response")


@pytest.fixture
def mock_model_with_tool_call():
    """Create a MockLlm that simulates a tool call response."""
    # Note: MockLlm currently only supports text responses
    # For tool call testing, we may need to extend the mock
    return MockLlm("tool_model", "I'll use the tool")


# =============================================================================
# Session Fixtures
# =============================================================================


@pytest.fixture
def session_service():
    """Create an in-memory session service."""
    return InMemorySessionService()


@pytest.fixture
def state():
    """Create an empty state object."""
    return State()


@pytest.fixture
def run_config():
    """Create a default run configuration."""
    return RunConfig()


# =============================================================================
# Agent Fixtures
# =============================================================================


@pytest.fixture
def simple_agent(mock_model):
    """Create a simple LlmAgent with mock model."""
    return (
        LlmAgent.builder("simple_agent")
        .model(mock_model)
        .instruction("You are a helpful assistant.")
        .build()
    )


@pytest.fixture
def agent_with_description(mock_model):
    """Create an LlmAgent with description."""
    return (
        LlmAgent.builder("described_agent")
        .model(mock_model)
        .description("A helpful assistant agent")
        .instruction("Help users with their questions.")
        .build()
    )


# =============================================================================
# Tool Fixtures
# =============================================================================


@pytest.fixture
def simple_tool():
    """Create a simple function tool."""

    def handler(ctx, args):
        return {"result": "tool executed"}

    return FunctionTool("simple_tool", "A simple test tool", handler)


@pytest.fixture
def math_tool():
    """Create a math tool that doubles numbers."""

    def handler(ctx, args):
        x = args.get("x", 0)
        return {"result": x * 2}

    schema = {
        "type": "object",
        "properties": {"x": {"type": "number", "description": "Number to double"}},
        "required": ["x"],
    }
    return FunctionTool("double", "Doubles a number", handler, schema)


@pytest.fixture
def async_tool():
    """Create an async function tool."""

    async def handler(ctx, args):
        return {"result": "async tool executed"}

    return FunctionTool("async_tool", "An async test tool", handler)


@pytest.fixture
def basic_toolset(simple_tool, math_tool):
    """Create a BasicToolset with multiple tools."""
    toolset = BasicToolset("test_toolset")
    toolset.add(simple_tool)
    toolset.add(math_tool)
    return toolset


# =============================================================================
# Memory & Artifact Fixtures
# =============================================================================


@pytest.fixture
def memory_service():
    """Create an in-memory memory service."""
    return InMemoryMemoryService()


@pytest.fixture
def artifact_service():
    """Create an in-memory artifact service."""
    return InMemoryArtifactService()


# =============================================================================
# Guardrail Fixtures
# =============================================================================


@pytest.fixture
def content_filter_harmful():
    """Create a content filter for harmful content."""
    return ContentFilter.harmful_content()


@pytest.fixture
def content_filter_max_length():
    """Create a content filter with max length."""
    return ContentFilter.max_length(100)


@pytest.fixture
def pii_redactor():
    """Create a PII redactor."""
    return PiiRedactor()


@pytest.fixture
def guardrail_set(content_filter_harmful, pii_redactor):
    """Create a guardrail set with content filter and PII redactor."""
    return (
        GuardrailSet().with_content_filter(content_filter_harmful).with_pii_redactor(pii_redactor)
    )


# =============================================================================
# Content Fixtures
# =============================================================================


@pytest.fixture
def user_content():
    """Create user content."""
    return Content.user("Hello, how are you?")


@pytest.fixture
def model_content():
    """Create model content."""
    return Content.model("I'm doing well, thank you!")


# =============================================================================
# Test Constants
# =============================================================================


APP_NAME = "test_app"
USER_ID = "test_user"
SESSION_ID = "test_session"
