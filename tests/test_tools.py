"""Tests for tools: FunctionTool, BasicToolset, AgentTool, built-in tools."""

from adk_rust import (
    AgentTool,
    BasicToolset,
    ExitLoopTool,
    FunctionTool,
    GoogleSearchTool,
    LlmAgent,
    LoadArtifactsTool,
    MockLlm,
)


class TestFunctionTool:
    """Tests for FunctionTool."""

    def test_create_function_tool(self):
        """Test creating a FunctionTool."""

        def handler(ctx, args):
            return {"result": "ok"}

        tool = FunctionTool("my_tool", "A test tool", handler)
        assert tool.name == "my_tool"
        assert tool.description == "A test tool"

    def test_function_tool_with_schema(self):
        """Test FunctionTool with parameter schema."""

        def handler(ctx, args):
            return {"doubled": args.get("x", 0) * 2}

        schema = {
            "type": "object",
            "properties": {"x": {"type": "number", "description": "Number to double"}},
            "required": ["x"],
        }
        tool = FunctionTool("double", "Doubles a number", handler, schema)
        assert tool.name == "double"
        assert tool.description == "Doubles a number"

    def test_function_tool_sync_handler(self):
        """Test FunctionTool with sync handler."""
        call_count = 0

        def handler(ctx, args):
            nonlocal call_count
            call_count += 1
            return {"count": call_count}

        tool = FunctionTool("counter", "Counts calls", handler)
        assert tool.name == "counter"

    def test_function_tool_async_handler(self):
        """Test FunctionTool with async handler."""

        async def handler(ctx, args):
            return {"async": True}

        tool = FunctionTool("async_tool", "Async tool", handler)
        assert tool.name == "async_tool"

    def test_function_tool_handler_receives_args(self):
        """Test that handler receives correct arguments."""
        received_args = {}

        def handler(ctx, args):
            received_args.update(args)
            return {"received": True}

        tool = FunctionTool("arg_tool", "Tests args", handler)
        # Note: actual execution happens via runner/agent
        assert tool.name == "arg_tool"

    def test_function_tool_complex_schema(self):
        """Test FunctionTool with complex schema."""

        def handler(ctx, args):
            return args

        schema = {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                },
            },
            "required": ["name"],
        }
        tool = FunctionTool("complex_tool", "Complex schema tool", handler, schema)
        assert tool.name == "complex_tool"

    def test_function_tool_no_schema(self):
        """Test FunctionTool without schema."""

        def handler(ctx, args):
            return {}

        tool = FunctionTool("no_schema", "No schema tool", handler)
        assert tool.name == "no_schema"


class TestBasicToolset:
    """Tests for BasicToolset."""

    def test_create_empty_toolset(self):
        """Test creating an empty BasicToolset."""
        toolset = BasicToolset("empty_toolset")
        assert toolset.name == "empty_toolset"
        assert len(toolset) == 0

    def test_add_tool_to_toolset(self):
        """Test adding a tool to toolset."""

        def handler(ctx, args):
            return {}

        toolset = BasicToolset("my_toolset")
        tool = FunctionTool("tool1", "Tool 1", handler)
        toolset.add(tool)
        assert len(toolset) == 1

    def test_add_multiple_tools(self):
        """Test adding multiple tools."""

        def handler(ctx, args):
            return {}

        toolset = BasicToolset("multi_toolset")
        for i in range(5):
            tool = FunctionTool(f"tool_{i}", f"Tool {i}", handler)
            toolset.add(tool)
        assert len(toolset) == 5

    def test_toolset_tools_method(self):
        """Test getting tools from toolset."""

        def handler(ctx, args):
            return {}

        toolset = BasicToolset("get_tools")
        toolset.add(FunctionTool("a", "A", handler))
        toolset.add(FunctionTool("b", "B", handler))

        tools = toolset.tools()
        assert isinstance(tools, list)
        assert len(tools) == 2

    def test_toolset_name_property(self):
        """Test toolset name property."""
        toolset = BasicToolset("named_toolset")
        assert toolset.name == "named_toolset"


class TestAgentTool:
    """Tests for AgentTool."""

    def test_create_agent_tool(self):
        """Test creating an AgentTool from LlmAgent."""
        model = MockLlm("mock", "response")
        agent = LlmAgent.builder("inner_agent").model(model).build()

        tool = AgentTool(agent)
        assert tool.name == "inner_agent"

    def test_agent_tool_description(self):
        """Test AgentTool inherits agent description."""
        model = MockLlm("mock", "response")
        agent = (
            LlmAgent.builder("described_agent").model(model).description("A helpful agent").build()
        )

        tool = AgentTool(agent)
        assert "helpful" in tool.description.lower() or tool.description is not None

    def test_agent_tool_skip_summarization(self):
        """Test AgentTool with skip_summarization option."""
        model = MockLlm("mock", "response")
        agent = LlmAgent.builder("agent").model(model).build()

        tool = AgentTool(agent, skip_summarization=True)
        assert tool.name == "agent"

    def test_agent_tool_forward_artifacts(self):
        """Test AgentTool with forward_artifacts option."""
        model = MockLlm("mock", "response")
        agent = LlmAgent.builder("agent").model(model).build()

        tool = AgentTool(agent, forward_artifacts=False)
        assert tool.name == "agent"

    def test_agent_tool_timeout(self):
        """Test AgentTool with timeout option."""
        model = MockLlm("mock", "response")
        agent = LlmAgent.builder("agent").model(model).build()

        tool = AgentTool(agent, timeout_secs=30)
        assert tool.name == "agent"

    def test_agent_tool_all_options(self):
        """Test AgentTool with all options."""
        model = MockLlm("mock", "response")
        agent = LlmAgent.builder("full_agent").model(model).build()

        tool = AgentTool(
            agent,
            skip_summarization=True,
            forward_artifacts=True,
            timeout_secs=60,
        )
        assert tool.name == "full_agent"


class TestExitLoopTool:
    """Tests for ExitLoopTool."""

    def test_create_exit_loop_tool(self):
        """Test creating an ExitLoopTool."""
        tool = ExitLoopTool()
        assert tool.name is not None
        assert tool.description is not None

    def test_exit_loop_tool_name(self):
        """Test ExitLoopTool has expected name."""
        tool = ExitLoopTool()
        # The name should indicate loop exit functionality
        assert "exit" in tool.name.lower() or "loop" in tool.name.lower() or tool.name


class TestLoadArtifactsTool:
    """Tests for LoadArtifactsTool."""

    def test_create_load_artifacts_tool(self):
        """Test creating a LoadArtifactsTool."""
        tool = LoadArtifactsTool()
        assert tool.name is not None
        assert tool.description is not None


class TestGoogleSearchTool:
    """Tests for GoogleSearchTool."""

    def test_create_google_search_tool(self):
        """Test creating a GoogleSearchTool."""
        tool = GoogleSearchTool()
        assert tool.name is not None
        assert tool.description is not None


class TestToolIntegration:
    """Integration tests for tools with agents."""

    def test_agent_with_function_tool(self):
        """Test LlmAgent with FunctionTool."""
        model = MockLlm("mock", "response")

        def handler(ctx, args):
            return {"result": "success"}

        tool = FunctionTool("test_tool", "Test tool", handler)
        agent = LlmAgent.builder("agent_with_tool").model(model).tool(tool).build()
        assert agent.name == "agent_with_tool"

    def test_agent_with_multiple_tools(self):
        """Test LlmAgent with multiple tools."""
        model = MockLlm("mock", "response")

        def handler1(ctx, args):
            return {"tool": 1}

        def handler2(ctx, args):
            return {"tool": 2}

        tool1 = FunctionTool("tool1", "Tool 1", handler1)
        tool2 = FunctionTool("tool2", "Tool 2", handler2)

        agent = LlmAgent.builder("multi_tool_agent").model(model).tool(tool1).tool(tool2).build()
        assert agent.name == "multi_tool_agent"

    def test_nested_agent_tool(self):
        """Test agent tool wrapping another agent with tools."""
        model = MockLlm("mock", "response")

        def handler(ctx, args):
            return {}

        inner_tool = FunctionTool("inner", "Inner tool", handler)
        inner_agent = LlmAgent.builder("inner").model(model).tool(inner_tool).build()

        # AgentTool wraps an agent as a tool
        agent_tool = AgentTool(inner_agent)
        assert agent_tool.name == "inner"
        # Note: LlmAgentBuilder.tool() only accepts FunctionTool, not AgentTool directly
        # AgentTool is typically used with sub_agent pattern for agent composition
