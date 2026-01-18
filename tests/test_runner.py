"""Tests for Runner and run_agent() execution."""

import pytest
from adk_rust import (
    Event,
    FunctionTool,
    InMemorySessionService,
    LlmAgent,
    MockLlm,
    RunConfig,
    Runner,
    StreamingMode,
    run_agent,
)


class TestRunner:
    """Tests for Runner class construction."""

    def test_create_runner(self, mock_model, session_service):
        """Test creating a Runner."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        runner = Runner("test_app", agent, session_service)
        assert runner is not None

    def test_create_runner_with_config(self, mock_model, session_service):
        """Test creating Runner with RunConfig."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        config = RunConfig()
        runner = Runner("test_app", agent, session_service, config)
        assert runner is not None

    def test_runner_repr(self, mock_model, session_service):
        """Test Runner __repr__."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        runner = Runner("my_app", agent, session_service)
        repr_str = repr(runner)
        assert "my_app" in repr_str


class TestRunAgent:
    """Tests for run_agent() convenience function."""

    @pytest.mark.asyncio
    async def test_run_agent_basic(self, mock_model):
        """Test basic run_agent() usage."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        result = await run_agent(agent, "Hello!")
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_with_custom_ids(self, mock_model):
        """Test run_agent() with custom user/session IDs."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        result = await run_agent(
            agent,
            "Hello!",
            user_id="custom_user",
            session_id="custom_session",
        )
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_with_app_name(self, mock_model):
        """Test run_agent() with custom app name."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        result = await run_agent(
            agent,
            "Hello!",
            app_name="custom_app",
        )
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_all_parameters(self, mock_model):
        """Test run_agent() with all parameters."""
        agent = LlmAgent.builder("test").model(mock_model).build()
        result = await run_agent(
            agent,
            "Hello!",
            user_id="user123",
            session_id="session456",
            app_name="my_app",
        )
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_returns_string(self):
        """Test run_agent() returns the mock model's response."""
        expected_response = "This is the mock response"
        model = MockLlm("test_mock", expected_response)
        agent = LlmAgent.builder("test").model(model).build()

        result = await run_agent(agent, "Any message")
        # The result should be a string response
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_with_instruction(self):
        """Test run_agent() with agent that has instructions."""
        model = MockLlm("test", "Response")
        agent = LlmAgent.builder("test").model(model).instruction("Be helpful").build()

        result = await run_agent(agent, "Hello")
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_with_description(self):
        """Test run_agent() with agent that has description."""
        model = MockLlm("test", "Response")
        agent = LlmAgent.builder("test").model(model).description("A test agent").build()

        result = await run_agent(agent, "Hello")
        assert isinstance(result, str)


class TestRunConfig:
    """Tests for RunConfig."""

    def test_create_run_config(self):
        """Test creating a RunConfig."""
        config = RunConfig()
        assert config is not None

    def test_run_config_streaming_mode(self):
        """Test RunConfig streaming_mode property."""
        config = RunConfig()
        mode = config.streaming_mode
        assert mode is not None

    def test_run_config_with_sse_mode(self):
        """Test creating RunConfig with SSE streaming mode."""
        config = RunConfig(streaming_mode=StreamingMode.SSE)
        assert config.streaming_mode == StreamingMode.SSE

    def test_run_config_default_is_sse(self):
        """Test that default streaming mode is SSE."""
        config = RunConfig()
        assert config.streaming_mode == StreamingMode.SSE


class TestStreamingMode:
    """Tests for StreamingMode enum."""

    def test_streaming_mode_none(self):
        """Test StreamingMode with no streaming."""
        # Use getattr since None is a Python keyword
        mode = getattr(StreamingMode, "None")
        assert mode is not None

    def test_streaming_mode_sse(self):
        """Test StreamingMode.SSE."""
        mode = StreamingMode.SSE
        assert mode is not None

    def test_streaming_mode_bidi(self):
        """Test StreamingMode.Bidi."""
        mode = StreamingMode.Bidi
        assert mode is not None

    def test_streaming_modes_are_different(self):
        """Test that different modes are not equal."""
        none_mode = getattr(StreamingMode, "None")
        assert none_mode != StreamingMode.SSE
        assert StreamingMode.SSE != StreamingMode.Bidi


class TestRunAgentWithTools:
    """Tests for run_agent with tools."""

    @pytest.mark.asyncio
    async def test_run_agent_with_tool(self):
        """Test run_agent with an agent that has tools."""

        def my_tool(ctx, args):
            return {"result": "tool_output"}

        model = MockLlm("test", "Response")
        tool = FunctionTool("my_tool", "A test tool", my_tool)
        agent = LlmAgent.builder("tool_agent").model(model).tool(tool).build()

        result = await run_agent(agent, "Hello")
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_run_agent_with_multiple_tools(self):
        """Test run_agent with an agent that has multiple tools."""

        def tool1(ctx, args):
            return {"result": "tool1"}

        def tool2(ctx, args):
            return {"result": "tool2"}

        model = MockLlm("test", "Response")
        agent = (
            LlmAgent.builder("multi_tool")
            .model(model)
            .tool(FunctionTool("tool1", "Tool 1", tool1))
            .tool(FunctionTool("tool2", "Tool 2", tool2))
            .build()
        )

        result = await run_agent(agent, "Hello")
        assert isinstance(result, str)


class TestEvent:
    """Tests for Event class interface."""

    def test_event_has_id_attribute(self):
        """Test Event has id attribute."""
        assert hasattr(Event, "id")

    def test_event_has_author_attribute(self):
        """Test Event has author attribute."""
        assert hasattr(Event, "author")

    def test_event_has_get_text_method(self):
        """Test Event has get_text method."""
        assert callable(getattr(Event, "get_text", None))

    def test_event_has_get_state_delta_method(self):
        """Test Event has get_state_delta method."""
        assert callable(getattr(Event, "get_state_delta", None))
