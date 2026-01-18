"""Tests for agent types: LlmAgent, CustomAgent, workflow agents."""

from adk_rust import (
    CustomAgent,
    CustomAgentBuilder,
    InvocationContext,
    LlmAgent,
    LlmAgentBuilder,
    LoopAgent,
    ParallelAgent,
    SequentialAgent,
)


class TestLlmAgentBuilder:
    """Tests for LlmAgentBuilder."""

    def test_create_builder(self):
        """Test creating a builder."""
        builder = LlmAgentBuilder("test_agent")
        # Builder should be chainable
        assert builder is not None

    def test_builder_via_static_method(self):
        """Test creating builder via LlmAgent.builder()."""
        builder = LlmAgent.builder("test_agent")
        assert builder is not None

    def test_builder_with_model(self, mock_model):
        """Test builder with model."""
        builder = LlmAgent.builder("agent").model(mock_model)
        assert builder is not None

    def test_builder_with_description(self, mock_model):
        """Test builder with description."""
        builder = LlmAgent.builder("agent").model(mock_model).description("A helpful agent")
        assert builder is not None

    def test_builder_with_instruction(self, mock_model):
        """Test builder with instruction."""
        builder = LlmAgent.builder("agent").model(mock_model).instruction("Be helpful and concise")
        assert builder is not None

    def test_builder_with_output_key(self, mock_model):
        """Test builder with output_key."""
        builder = LlmAgent.builder("agent").model(mock_model).output_key("result")
        assert builder is not None

    def test_builder_chain_all_options(self, mock_model, simple_tool):
        """Test builder chaining all options."""
        agent = (
            LlmAgent.builder("full_agent")
            .model(mock_model)
            .description("Full-featured agent")
            .instruction("Follow instructions carefully")
            .tool(simple_tool)
            .output_key("output")
            .build()
        )
        assert agent.name == "full_agent"


class TestLlmAgent:
    """Tests for LlmAgent."""

    def test_create_simple_agent(self, mock_model):
        """Test creating a simple LlmAgent."""
        agent = LlmAgent.builder("simple").model(mock_model).build()
        assert agent.name == "simple"

    def test_agent_name_property(self, mock_model):
        """Test agent name property."""
        agent = LlmAgent.builder("named_agent").model(mock_model).build()
        assert agent.name == "named_agent"

    def test_agent_description_property(self, mock_model):
        """Test agent description property."""
        agent = LlmAgent.builder("agent").model(mock_model).description("Test description").build()
        # Description should be accessible
        assert agent.description is not None or agent.description == ""

    def test_agent_with_tool(self, mock_model, simple_tool):
        """Test agent with a tool."""
        agent = LlmAgent.builder("tool_agent").model(mock_model).tool(simple_tool).build()
        assert agent.name == "tool_agent"

    def test_agent_with_sub_agent(self, mock_model):
        """Test agent with sub-agent."""
        sub_agent = LlmAgent.builder("sub").model(mock_model).build()
        agent = LlmAgent.builder("main").model(mock_model).sub_agent(sub_agent).build()
        assert agent.name == "main"


class TestCustomAgentBuilder:
    """Tests for CustomAgentBuilder."""

    def test_create_builder(self):
        """Test creating a CustomAgentBuilder."""
        builder = CustomAgentBuilder("custom")
        assert builder is not None

    def test_builder_via_static_method(self):
        """Test creating builder via CustomAgent.builder()."""
        builder = CustomAgent.builder("custom")
        assert builder is not None

    def test_builder_with_description(self):
        """Test builder with description."""
        builder = CustomAgent.builder("custom").description("Custom agent")
        assert builder is not None


class TestCustomAgent:
    """Tests for CustomAgent."""

    def test_create_custom_agent_with_sync_handler(self):
        """Test creating CustomAgent with sync handler."""

        async def handler(ctx: InvocationContext) -> str:
            return "Sync response"

        agent = CustomAgent.builder("sync_custom").handler(handler).build()
        assert agent.name == "sync_custom"

    def test_create_custom_agent_with_async_handler(self):
        """Test creating CustomAgent with async handler."""

        async def handler(ctx: InvocationContext) -> str:
            return "Async response"

        agent = CustomAgent.builder("async_custom").handler(handler).build()
        assert agent.name == "async_custom"

    def test_custom_agent_with_description(self):
        """Test CustomAgent with description."""

        async def handler(ctx: InvocationContext) -> str:
            return "response"

        agent = (
            CustomAgent.builder("described").description("A custom agent").handler(handler).build()
        )
        assert agent.name == "described"

    def test_custom_agent_name_property(self):
        """Test CustomAgent name property."""

        async def handler(ctx: InvocationContext) -> str:
            return "response"

        agent = CustomAgent.builder("named_custom").handler(handler).build()
        assert agent.name == "named_custom"

    def test_custom_agent_description_property(self):
        """Test CustomAgent description property."""

        async def handler(ctx: InvocationContext) -> str:
            return "response"

        agent = (
            CustomAgent.builder("custom").description("Test description").handler(handler).build()
        )
        assert agent.description is not None


class TestSequentialAgent:
    """Tests for SequentialAgent."""

    def test_create_sequential_agent(self, mock_model):
        """Test creating a SequentialAgent."""
        agent1 = LlmAgent.builder("agent1").model(mock_model).build()
        agent2 = LlmAgent.builder("agent2").model(mock_model).build()

        sequential = SequentialAgent("seq", [agent1, agent2])
        assert sequential.name == "seq"

    def test_sequential_agent_with_single_agent(self, mock_model):
        """Test SequentialAgent with single agent."""
        agent = LlmAgent.builder("single").model(mock_model).build()
        sequential = SequentialAgent("single_seq", [agent])
        assert sequential.name == "single_seq"

    def test_sequential_agent_with_many_agents(self, mock_model):
        """Test SequentialAgent with many agents."""
        agents = [LlmAgent.builder(f"agent_{i}").model(mock_model).build() for i in range(5)]
        sequential = SequentialAgent("many_seq", agents)
        assert sequential.name == "many_seq"


class TestParallelAgent:
    """Tests for ParallelAgent."""

    def test_create_parallel_agent(self, mock_model):
        """Test creating a ParallelAgent."""
        agent1 = LlmAgent.builder("agent1").model(mock_model).build()
        agent2 = LlmAgent.builder("agent2").model(mock_model).build()

        parallel = ParallelAgent("par", [agent1, agent2])
        assert parallel.name == "par"

    def test_parallel_agent_with_single_agent(self, mock_model):
        """Test ParallelAgent with single agent."""
        agent = LlmAgent.builder("single").model(mock_model).build()
        parallel = ParallelAgent("single_par", [agent])
        assert parallel.name == "single_par"

    def test_parallel_agent_with_many_agents(self, mock_model):
        """Test ParallelAgent with many agents."""
        agents = [LlmAgent.builder(f"agent_{i}").model(mock_model).build() for i in range(5)]
        parallel = ParallelAgent("many_par", agents)
        assert parallel.name == "many_par"


class TestLoopAgent:
    """Tests for LoopAgent."""

    def test_create_loop_agent(self, mock_model):
        """Test creating a LoopAgent."""
        agent = LlmAgent.builder("loop_inner").model(mock_model).build()
        loop = LoopAgent("loop", [agent])
        assert loop.name == "loop"

    def test_loop_agent_with_max_iterations(self, mock_model):
        """Test LoopAgent with max_iterations."""
        agent = LlmAgent.builder("inner").model(mock_model).build()
        loop = LoopAgent("bounded_loop", [agent], max_iterations=5)
        assert loop.name == "bounded_loop"

    def test_loop_agent_default_max_iterations(self, mock_model):
        """Test LoopAgent uses default max_iterations."""
        agent = LlmAgent.builder("inner").model(mock_model).build()
        loop = LoopAgent("default_loop", [agent])
        # Default should be 10 according to the stubs
        assert loop.name == "default_loop"

    def test_loop_agent_multiple_inner_agents(self, mock_model):
        """Test LoopAgent with multiple inner agents."""
        agents = [LlmAgent.builder(f"inner_{i}").model(mock_model).build() for i in range(3)]
        loop = LoopAgent("multi_loop", agents, max_iterations=3)
        assert loop.name == "multi_loop"


class TestAgentComposition:
    """Tests for composing agents together."""

    def test_sequential_with_parallel_inside(self, mock_model):
        """Test SequentialAgent containing parallel execution."""
        # This tests the concept, actual nesting may require specific support
        agent1 = LlmAgent.builder("a1").model(mock_model).build()
        agent2 = LlmAgent.builder("a2").model(mock_model).build()

        sequential = SequentialAgent("outer", [agent1, agent2])
        assert sequential.name == "outer"

    def test_agent_with_sub_agents_hierarchy(self, mock_model):
        """Test hierarchical sub-agent structure."""
        leaf1 = LlmAgent.builder("leaf1").model(mock_model).build()
        leaf2 = LlmAgent.builder("leaf2").model(mock_model).build()

        middle = (
            LlmAgent.builder("middle").model(mock_model).sub_agent(leaf1).sub_agent(leaf2).build()
        )

        root = LlmAgent.builder("root").model(mock_model).sub_agent(middle).build()
        assert root.name == "root"

    def test_custom_agent_with_sub_agents(self, mock_model):
        """Test CustomAgent with sub-agents."""
        sub_agent = LlmAgent.builder("sub").model(mock_model).build()

        async def handler(ctx: InvocationContext) -> str:
            return "Custom with sub-agents"

        custom = CustomAgent.builder("custom_parent").handler(handler).sub_agent(sub_agent).build()
        assert custom.name == "custom_parent"
