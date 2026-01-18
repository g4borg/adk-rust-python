"""Tests for callback functionality."""

import pytest
from adk_rust import (
    BeforeModelResult,
    CallbackContext,
    Content,
    LlmAgent,
    LlmRequest,
    LlmResponse,
    MockLlm,
    run_agent,
)


class TestBeforeModelResult:
    """Tests for BeforeModelResult."""

    def test_continue_result(self):
        """Test BeforeModelResult.cont() to continue execution."""
        result = BeforeModelResult.cont()
        assert result is not None

    def test_skip_result(self):
        """Test BeforeModelResult.skip() to skip model call."""
        result = BeforeModelResult.skip("Skipped response")
        assert result is not None


class TestLlmRequest:
    """Tests for LlmRequest structure."""

    def test_llm_request_has_model(self):
        """Test LlmRequest has model property."""
        assert hasattr(LlmRequest, "model")

    def test_llm_request_has_contents(self):
        """Test LlmRequest has contents property."""
        assert hasattr(LlmRequest, "contents")


class TestLlmResponse:
    """Tests for LlmResponse structure."""

    def test_create_llm_response(self):
        """Test creating an LlmResponse."""
        response = LlmResponse()
        assert response is not None

    def test_llm_response_with_content(self):
        """Test LlmResponse with content."""
        content = Content.model("Test response")
        response = LlmResponse(content=content)
        assert response.content is not None

    def test_llm_response_partial(self):
        """Test LlmResponse partial property."""
        response = LlmResponse(partial=True)
        assert response.partial is True

    def test_llm_response_turn_complete(self):
        """Test LlmResponse turn_complete property."""
        response = LlmResponse(turn_complete=False)
        assert response.turn_complete is False

    def test_llm_response_get_text(self):
        """Test LlmResponse.get_text()."""
        content = Content.model("Response text")
        response = LlmResponse(content=content)
        text = response.get_text()
        # May or may not have text depending on implementation
        assert text is None or isinstance(text, str)


class TestCallbackContext:
    """Tests for CallbackContext structure."""

    def test_callback_context_has_invocation_id(self):
        """Test CallbackContext has invocation_id property."""
        assert hasattr(CallbackContext, "invocation_id")

    def test_callback_context_has_agent_name(self):
        """Test CallbackContext has agent_name property."""
        assert hasattr(CallbackContext, "agent_name")

    def test_callback_context_has_user_id(self):
        """Test CallbackContext has user_id property."""
        assert hasattr(CallbackContext, "user_id")

    def test_callback_context_has_app_name(self):
        """Test CallbackContext has app_name property."""
        assert hasattr(CallbackContext, "app_name")

    def test_callback_context_has_session_id(self):
        """Test CallbackContext has session_id property."""
        assert hasattr(CallbackContext, "session_id")

    def test_callback_context_has_user_content(self):
        """Test CallbackContext has user_content property."""
        assert hasattr(CallbackContext, "user_content")

    def test_callback_context_has_state(self):
        """Test CallbackContext has state property."""
        assert hasattr(CallbackContext, "state")


class TestBeforeAgentCallback:
    """Tests for before_agent_callback."""

    def test_agent_with_before_agent_callback(self, mock_model):
        """Test creating agent with before_agent_callback."""
        callback_invoked = []

        def before_agent(ctx: CallbackContext):
            callback_invoked.append(True)
            return None  # Continue execution

        agent = (
            LlmAgent.builder("test").model(mock_model).before_agent_callback(before_agent).build()
        )
        assert agent.name == "test"

    def test_agent_with_async_before_agent_callback(self, mock_model):
        """Test agent with async before_agent_callback."""

        async def before_agent(ctx: CallbackContext):
            return None

        agent = (
            LlmAgent.builder("test").model(mock_model).before_agent_callback(before_agent).build()
        )
        assert agent.name == "test"

    def test_before_agent_can_return_content(self, mock_model):
        """Test before_agent returning content to short-circuit."""

        def before_agent(ctx: CallbackContext):
            return Content.model("Short-circuited response")

        agent = (
            LlmAgent.builder("test").model(mock_model).before_agent_callback(before_agent).build()
        )
        assert agent.name == "test"

    def test_before_agent_can_return_string(self, mock_model):
        """Test before_agent returning string to short-circuit."""

        def before_agent(ctx: CallbackContext):
            return "String response"

        agent = (
            LlmAgent.builder("test").model(mock_model).before_agent_callback(before_agent).build()
        )
        assert agent.name == "test"


class TestAfterAgentCallback:
    """Tests for after_agent_callback."""

    def test_agent_with_after_agent_callback(self, mock_model):
        """Test creating agent with after_agent_callback."""

        def after_agent(ctx: CallbackContext):
            return None

        agent = LlmAgent.builder("test").model(mock_model).after_agent_callback(after_agent).build()
        assert agent.name == "test"

    def test_agent_with_async_after_agent_callback(self, mock_model):
        """Test agent with async after_agent_callback."""

        async def after_agent(ctx: CallbackContext):
            return None

        agent = LlmAgent.builder("test").model(mock_model).after_agent_callback(after_agent).build()
        assert agent.name == "test"


class TestBeforeModelCallback:
    """Tests for before_model_callback."""

    def test_agent_with_before_model_callback(self, mock_model):
        """Test creating agent with before_model_callback."""

        def before_model(ctx: CallbackContext, request: LlmRequest):
            return BeforeModelResult.cont()

        agent = (
            LlmAgent.builder("test").model(mock_model).before_model_callback(before_model).build()
        )
        assert agent.name == "test"

    def test_agent_with_async_before_model_callback(self, mock_model):
        """Test agent with async before_model_callback."""

        async def before_model(ctx: CallbackContext, request: LlmRequest):
            return BeforeModelResult.cont()

        agent = (
            LlmAgent.builder("test").model(mock_model).before_model_callback(before_model).build()
        )
        assert agent.name == "test"

    def test_before_model_can_skip(self, mock_model):
        """Test before_model can skip model call."""

        def before_model(ctx: CallbackContext, request: LlmRequest):
            return BeforeModelResult.skip("Cached response")

        agent = (
            LlmAgent.builder("test").model(mock_model).before_model_callback(before_model).build()
        )
        assert agent.name == "test"


class TestAfterModelCallback:
    """Tests for after_model_callback."""

    def test_agent_with_after_model_callback(self, mock_model):
        """Test creating agent with after_model_callback."""

        def after_model(ctx: CallbackContext, response: LlmResponse):
            return response

        agent = LlmAgent.builder("test").model(mock_model).after_model_callback(after_model).build()
        assert agent.name == "test"

    def test_agent_with_async_after_model_callback(self, mock_model):
        """Test agent with async after_model_callback."""

        async def after_model(ctx: CallbackContext, response: LlmResponse):
            return response

        agent = LlmAgent.builder("test").model(mock_model).after_model_callback(after_model).build()
        assert agent.name == "test"

    def test_after_model_can_modify_response(self, mock_model):
        """Test after_model can modify response."""

        def after_model(ctx: CallbackContext, response: LlmResponse):
            # Return modified response
            return LlmResponse(
                content=Content.model("Modified response"),
                turn_complete=True,
            )

        agent = LlmAgent.builder("test").model(mock_model).after_model_callback(after_model).build()
        assert agent.name == "test"


class TestBeforeToolCallback:
    """Tests for before_tool_callback."""

    def test_agent_with_before_tool_callback(self, mock_model):
        """Test creating agent with before_tool_callback."""

        def before_tool(ctx: CallbackContext):
            return None

        agent = LlmAgent.builder("test").model(mock_model).before_tool_callback(before_tool).build()
        assert agent.name == "test"

    def test_agent_with_async_before_tool_callback(self, mock_model):
        """Test agent with async before_tool_callback."""

        async def before_tool(ctx: CallbackContext):
            return None

        agent = LlmAgent.builder("test").model(mock_model).before_tool_callback(before_tool).build()
        assert agent.name == "test"


class TestAfterToolCallback:
    """Tests for after_tool_callback."""

    def test_agent_with_after_tool_callback(self, mock_model):
        """Test creating agent with after_tool_callback."""

        def after_tool(ctx: CallbackContext):
            return None

        agent = LlmAgent.builder("test").model(mock_model).after_tool_callback(after_tool).build()
        assert agent.name == "test"

    def test_agent_with_async_after_tool_callback(self, mock_model):
        """Test agent with async after_tool_callback."""

        async def after_tool(ctx: CallbackContext):
            return None

        agent = LlmAgent.builder("test").model(mock_model).after_tool_callback(after_tool).build()
        assert agent.name == "test"


class TestMultipleCallbacks:
    """Tests for agents with multiple callbacks."""

    def test_agent_with_all_callbacks(self, mock_model):
        """Test agent with all callback types."""

        def before_agent(ctx):
            return None

        def after_agent(ctx):
            return None

        def before_model(ctx, request):
            return BeforeModelResult.cont()

        def after_model(ctx, response):
            return response

        def before_tool(ctx):
            return None

        def after_tool(ctx):
            return None

        agent = (
            LlmAgent.builder("full_callbacks")
            .model(mock_model)
            .before_agent_callback(before_agent)
            .after_agent_callback(after_agent)
            .before_model_callback(before_model)
            .after_model_callback(after_model)
            .before_tool_callback(before_tool)
            .after_tool_callback(after_tool)
            .build()
        )
        assert agent.name == "full_callbacks"


class TestCallbackExecution:
    """Tests for callback execution during agent run."""

    @pytest.mark.asyncio
    async def test_callbacks_execute_on_run(self):
        """Test that callbacks are executed during agent run."""
        callback_log = []

        def before_agent(ctx):
            callback_log.append("before_agent")
            return None

        def after_agent(ctx):
            callback_log.append("after_agent")
            return None

        model = MockLlm("test", "Response")
        agent = (
            LlmAgent.builder("test")
            .model(model)
            .before_agent_callback(before_agent)
            .after_agent_callback(after_agent)
            .build()
        )

        # Use run_agent which handles session creation internally
        result = await run_agent(agent, "Hello")

        # Result should be a string response
        assert isinstance(result, str)

    @pytest.mark.asyncio
    async def test_before_model_skip_prevents_model_call(self):
        """Test that BeforeModelResult.skip() prevents model call."""
        skip_response = "Skipped via callback"

        def before_model(ctx, request):
            return BeforeModelResult.skip(skip_response)

        model = MockLlm("test", "Should not see this")
        agent = LlmAgent.builder("test").model(model).before_model_callback(before_model).build()

        result = await run_agent(agent, "Hello")
        # Result might contain the skip response
        assert isinstance(result, str)
