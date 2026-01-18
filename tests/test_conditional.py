"""Tests for conditional agents: ConditionalAgent, LlmConditionalAgent."""

from adk_rust import (
    ConditionalAgent,
    CustomAgent,
    InvocationContext,
    LlmAgent,
    LlmConditionalAgent,
)


class TestConditionalAgent:
    """Tests for ConditionalAgent (rule-based routing)."""

    def test_create_conditional_agent(self, mock_model):
        """Test creating a conditional agent."""
        if_agent = LlmAgent.builder("if_branch").model(mock_model).build()
        else_agent = LlmAgent.builder("else_branch").model(mock_model).build()

        def condition(ctx: InvocationContext) -> bool:
            return True

        agent = ConditionalAgent(
            "router",
            condition=condition,
            if_agent=if_agent,
            else_agent=else_agent,
        )
        assert agent.name == "router"

    def test_conditional_agent_without_else(self, mock_model):
        """Test conditional agent without else branch."""
        if_agent = LlmAgent.builder("if_branch").model(mock_model).build()

        def condition(ctx: InvocationContext) -> bool:
            return True

        agent = ConditionalAgent(
            "router",
            condition=condition,
            if_agent=if_agent,
        )
        assert agent.name == "router"

    def test_conditional_agent_with_description(self, mock_model):
        """Test conditional agent with description."""
        if_agent = LlmAgent.builder("if_branch").model(mock_model).build()

        def condition(ctx: InvocationContext) -> bool:
            return False

        agent = ConditionalAgent(
            "router",
            condition=condition,
            if_agent=if_agent,
            description="Routes based on condition",
        )
        assert agent.name == "router"
        assert agent.description == "Routes based on condition"

    def test_conditional_agent_condition_true(self, mock_model):
        """Test conditional agent when condition returns True."""
        true_agent = LlmAgent.builder("true_path").model(mock_model).build()
        false_agent = LlmAgent.builder("false_path").model(mock_model).build()

        def always_true(ctx: InvocationContext) -> bool:
            return True

        agent = ConditionalAgent(
            "router",
            condition=always_true,
            if_agent=true_agent,
            else_agent=false_agent,
        )
        assert agent.name == "router"

    def test_conditional_agent_condition_false(self, mock_model):
        """Test conditional agent when condition returns False."""
        true_agent = LlmAgent.builder("true_path").model(mock_model).build()
        false_agent = LlmAgent.builder("false_path").model(mock_model).build()

        def always_false(ctx: InvocationContext) -> bool:
            return False

        agent = ConditionalAgent(
            "router",
            condition=always_false,
            if_agent=true_agent,
            else_agent=false_agent,
        )
        assert agent.name == "router"

    def test_conditional_agent_with_state_check(self, mock_model):
        """Test conditional agent checking state."""
        premium_agent = LlmAgent.builder("premium").model(mock_model).build()
        basic_agent = LlmAgent.builder("basic").model(mock_model).build()

        def is_premium(ctx: InvocationContext) -> bool:
            return ctx.state.get("premium") is True

        agent = ConditionalAgent(
            "tier_router",
            condition=is_premium,
            if_agent=premium_agent,
            else_agent=basic_agent,
        )
        assert agent.name == "tier_router"

    def test_conditional_agent_with_custom_agents(self, mock_model):
        """Test conditional agent with CustomAgent branches."""

        async def if_handler(ctx: InvocationContext) -> str:
            return "If branch response"

        async def else_handler(ctx: InvocationContext) -> str:
            return "Else branch response"

        if_agent = CustomAgent.builder("if_custom").handler(if_handler).build()
        else_agent = CustomAgent.builder("else_custom").handler(else_handler).build()

        def condition(ctx: InvocationContext) -> bool:
            return True

        agent = ConditionalAgent(
            "custom_router",
            condition=condition,
            if_agent=if_agent,
            else_agent=else_agent,
        )
        assert agent.name == "custom_router"


class TestLlmConditionalAgentBuilder:
    """Tests for LlmConditionalAgentBuilder."""

    def test_create_builder(self, mock_model):
        """Test creating a builder."""
        builder = LlmConditionalAgent.builder("router", mock_model)
        assert builder is not None

    def test_builder_with_description(self, mock_model):
        """Test builder with description."""
        builder = LlmConditionalAgent.builder("router", mock_model).description(
            "Intelligent router"
        )
        assert builder is not None

    def test_builder_with_instruction(self, mock_model):
        """Test builder with instruction."""
        builder = LlmConditionalAgent.builder("router", mock_model).instruction(
            "Classify user requests"
        )
        assert builder is not None

    def test_builder_add_route(self, mock_model):
        """Test adding routes to builder."""
        tech_agent = LlmAgent.builder("tech").model(mock_model).build()

        builder = LlmConditionalAgent.builder("router", mock_model).route("technical", tech_agent)
        assert builder is not None

    def test_builder_multiple_routes(self, mock_model):
        """Test adding multiple routes."""
        tech_agent = LlmAgent.builder("tech").model(mock_model).build()
        billing_agent = LlmAgent.builder("billing").model(mock_model).build()
        general_agent = LlmAgent.builder("general").model(mock_model).build()

        builder = (
            LlmConditionalAgent.builder("router", mock_model)
            .route("technical", tech_agent)
            .route("billing", billing_agent)
            .route("general", general_agent)
        )
        assert builder is not None

    def test_builder_with_default_route(self, mock_model):
        """Test builder with default route."""
        fallback_agent = LlmAgent.builder("fallback").model(mock_model).build()

        builder = LlmConditionalAgent.builder("router", mock_model).default_route(fallback_agent)
        assert builder is not None

    def test_builder_build(self, mock_model):
        """Test building the agent."""
        tech_agent = LlmAgent.builder("tech").model(mock_model).build()
        fallback_agent = LlmAgent.builder("fallback").model(mock_model).build()

        agent = (
            LlmConditionalAgent.builder("router", mock_model)
            .instruction("Classify as 'technical' or 'other'")
            .route("technical", tech_agent)
            .default_route(fallback_agent)
            .build()
        )
        assert agent.name == "router"


class TestLlmConditionalAgent:
    """Tests for LlmConditionalAgent."""

    def test_create_full_router(self, mock_model):
        """Test creating a full LLM-based router."""
        tech_agent = LlmAgent.builder("tech_support").model(mock_model).build()
        billing_agent = LlmAgent.builder("billing_support").model(mock_model).build()
        general_agent = LlmAgent.builder("general_support").model(mock_model).build()

        router = (
            LlmConditionalAgent.builder("support_router", mock_model)
            .description("Routes support requests to appropriate teams")
            .instruction("Classify the user's request as 'technical', 'billing', or 'general'")
            .route("technical", tech_agent)
            .route("billing", billing_agent)
            .default_route(general_agent)
            .build()
        )

        assert router.name == "support_router"

    def test_router_name_property(self, mock_model):
        """Test router name property."""
        agent = LlmAgent.builder("inner").model(mock_model).build()

        router = (
            LlmConditionalAgent.builder("named_router", mock_model)
            .instruction("Classify the request")
            .route("route1", agent)
            .build()
        )
        assert router.name == "named_router"

    def test_router_description_property(self, mock_model):
        """Test router description property."""
        agent = LlmAgent.builder("inner").model(mock_model).build()

        router = (
            LlmConditionalAgent.builder("router", mock_model)
            .description("My router description")
            .instruction("Classify the request")
            .route("route1", agent)
            .build()
        )
        assert router.description is not None

    def test_router_with_custom_agent_routes(self, mock_model):
        """Test router with CustomAgent routes."""

        async def handler1(ctx: InvocationContext) -> str:
            return "Route 1 response"

        async def handler2(ctx: InvocationContext) -> str:
            return "Route 2 response"

        custom1 = CustomAgent.builder("custom1").handler(handler1).build()
        custom2 = CustomAgent.builder("custom2").handler(handler2).build()

        router = (
            LlmConditionalAgent.builder("custom_router", mock_model)
            .instruction("Route to custom1 or custom2")
            .route("custom1", custom1)
            .route("custom2", custom2)
            .build()
        )
        assert router.name == "custom_router"

    def test_router_with_mixed_agent_types(self, mock_model):
        """Test router with both LlmAgent and CustomAgent routes."""

        async def custom_handler(ctx: InvocationContext) -> str:
            return "Custom response"

        llm_agent = LlmAgent.builder("llm").model(mock_model).build()
        custom_agent = CustomAgent.builder("custom").handler(custom_handler).build()

        router = (
            LlmConditionalAgent.builder("mixed_router", mock_model)
            .instruction("Route to llm or custom")
            .route("llm", llm_agent)
            .route("custom", custom_agent)
            .build()
        )
        assert router.name == "mixed_router"


class TestConditionalAgentIntegration:
    """Integration tests for conditional agents."""

    def test_nested_conditional_agents(self, mock_model):
        """Test nesting conditional agents."""
        leaf1 = LlmAgent.builder("leaf1").model(mock_model).build()
        leaf2 = LlmAgent.builder("leaf2").model(mock_model).build()
        leaf3 = LlmAgent.builder("leaf3").model(mock_model).build()

        # Inner conditional
        inner = ConditionalAgent(
            "inner_router",
            condition=lambda ctx: ctx.state.get("option") == "a",
            if_agent=leaf1,
            else_agent=leaf2,
        )

        # Outer conditional
        outer = ConditionalAgent(
            "outer_router",
            condition=lambda ctx: ctx.state.get("use_inner") is True,
            if_agent=inner,
            else_agent=leaf3,
        )

        assert outer.name == "outer_router"

    def test_llm_router_as_sub_agent(self, mock_model):
        """Test using LlmConditionalAgent as sub-agent."""
        tech_agent = LlmAgent.builder("tech").model(mock_model).build()
        general_agent = LlmAgent.builder("general").model(mock_model).build()

        router = (
            LlmConditionalAgent.builder("router", mock_model)
            .instruction("Route to tech or general")
            .route("tech", tech_agent)
            .default_route(general_agent)
            .build()
        )

        # Use router as tool in another agent (via AgentTool)
        # This tests that the router is a valid agent
        assert router.name == "router"

    def test_complex_routing_setup(self, mock_model):
        """Test complex multi-level routing."""
        # Tier 1 agents
        premium_support = LlmAgent.builder("premium_support").model(mock_model).build()
        basic_support = LlmAgent.builder("basic_support").model(mock_model).build()

        # Tier selector (rule-based)
        tier_router = ConditionalAgent(
            "tier_router",
            condition=lambda ctx: ctx.state.get("tier") == "premium",
            if_agent=premium_support,
            else_agent=basic_support,
        )

        # Category agents
        sales_agent = LlmAgent.builder("sales").model(mock_model).build()

        # Top-level router (LLM-based)
        main_router = (
            LlmConditionalAgent.builder("main_router", mock_model)
            .instruction("Classify as 'support' or 'sales'")
            .route("support", tier_router)
            .route("sales", sales_agent)
            .build()
        )

        assert main_router.name == "main_router"
