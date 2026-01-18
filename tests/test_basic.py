"""Basic tests for adk_rust Python bindings."""



class TestImports:
    """Test that all exports are importable."""

    def test_import_adk_rust(self):
        import adk_rust
        assert hasattr(adk_rust, '__version__')

    def test_import_types(self):
        from adk_rust import Content, Event, Part
        assert Content is not None
        assert Event is not None
        assert Part is not None

    def test_import_agents(self):
        from adk_rust import LlmAgent, LlmAgentBuilder
        assert LlmAgent is not None
        assert LlmAgentBuilder is not None

    def test_import_models(self):
        from adk_rust import GeminiModel
        assert GeminiModel is not None

    def test_import_tools(self):
        from adk_rust import FunctionTool, BasicToolset
        assert FunctionTool is not None
        assert BasicToolset is not None

    def test_import_runner(self):
        from adk_rust import Runner
        assert Runner is not None


class TestContent:
    """Test Content type."""

    def test_create_user_content(self):
        from adk_rust import Content
        content = Content.user("Hello, world!")
        assert content.role == "user"
        assert content.get_text() == "Hello, world!"

    def test_create_model_content(self):
        from adk_rust import Content
        content = Content.model("Hi there!")
        assert content.role == "model"
        assert content.get_text() == "Hi there!"


class TestPart:
    """Test Part type."""

    def test_create_text_part(self):
        from adk_rust import Part
        part = Part.text("Hello")
        assert part.is_text()
        assert not part.is_function_call()
        assert part.get_text() == "Hello"


class TestFunctionTool:
    """Test FunctionTool."""

    def test_create_function_tool(self):
        from adk_rust import FunctionTool

        def my_tool(ctx, args):
            return {"result": "ok"}

        tool = FunctionTool("my_tool", "A test tool", my_tool)
        assert tool.name == "my_tool"
        assert tool.description == "A test tool"


class TestBasicToolset:
    """Test BasicToolset."""

    def test_create_toolset(self):
        from adk_rust import BasicToolset, FunctionTool

        def tool1(ctx, args):
            return {}

        def tool2(ctx, args):
            return {}

        toolset = BasicToolset("my_toolset")
        toolset.add(FunctionTool("tool1", "Tool 1", tool1))
        toolset.add(FunctionTool("tool2", "Tool 2", tool2))

        assert len(toolset) == 2
        assert toolset.name == "my_toolset"
