"""Type stubs for adk_rust"""

from typing import Any, Awaitable, Callable, Dict, List, Optional

# Types
class Content:
    role: str
    parts: List[Part]

    def __init__(self, role: str = "user") -> None: ...
    @staticmethod
    def user(text: str) -> Content: ...
    @staticmethod
    def model(text: str) -> Content: ...
    def with_text(self, text: str) -> Content: ...
    def get_text(self) -> Optional[str]: ...

class Part:
    @staticmethod
    def text(text: str) -> Part: ...
    def get_text(self) -> Optional[str]: ...
    def is_text(self) -> bool: ...
    def is_function_call(self) -> bool: ...

class Event:
    id: str
    author: str
    def content(self) -> Optional[Content]: ...
    def get_text(self) -> Optional[str]: ...
    def get_state_delta(self) -> Dict[str, Any]: ...
    def transfer_to_agent(self) -> Optional[str]: ...

# Models
class GeminiModel:
    def __init__(self, api_key: str, model: str = "gemini-2.5-flash") -> None: ...
    @property
    def name(self) -> str: ...

class OpenAIModel:
    def __init__(self, api_key: str, model: str = "gpt-4o") -> None: ...
    @staticmethod
    def compatible(api_key: str, base_url: str, model: str) -> OpenAIModel: ...
    @property
    def name(self) -> str: ...

class AnthropicModel:
    def __init__(self, api_key: str, model: str = "claude-sonnet-4-20250514") -> None: ...
    @staticmethod
    def from_api_key(api_key: str) -> AnthropicModel: ...
    @property
    def name(self) -> str: ...

class DeepSeekModel:
    def __init__(self, api_key: str, model: str) -> None: ...
    @staticmethod
    def chat(api_key: str) -> DeepSeekModel: ...
    @staticmethod
    def reasoner(api_key: str) -> DeepSeekModel: ...
    @property
    def name(self) -> str: ...

class GroqModel:
    def __init__(self, api_key: str, model: str) -> None: ...
    @staticmethod
    def llama70b(api_key: str) -> GroqModel: ...
    @staticmethod
    def llama8b(api_key: str) -> GroqModel: ...
    @staticmethod
    def mixtral(api_key: str) -> GroqModel: ...
    @property
    def name(self) -> str: ...

class OllamaModel:
    def __init__(self, model: str) -> None: ...
    @staticmethod
    def with_host(host: str, model: str) -> OllamaModel: ...
    @property
    def name(self) -> str: ...

class MockLlm:
    def __init__(self, name: str, response_text: str = "Mock response") -> None: ...
    @property
    def name(self) -> str: ...

# Agents
class LlmAgentBuilder:
    def __init__(self, name: str) -> None: ...
    def description(self, desc: str) -> LlmAgentBuilder: ...
    def instruction(self, instruction: str) -> LlmAgentBuilder: ...
    def model(self, model: Any) -> LlmAgentBuilder: ...
    def tool(self, tool: FunctionTool) -> LlmAgentBuilder: ...
    def mcp_tool(self, tool: McpTool) -> LlmAgentBuilder:
        """Add an MCP tool (from McpToolset.get_tools()) to the agent."""
        ...
    def sub_agent(self, agent: LlmAgent) -> LlmAgentBuilder: ...
    def output_key(self, key: str) -> LlmAgentBuilder: ...
    def before_agent_callback(
        self,
        callback: Callable[
            [CallbackContext], Awaitable[Optional[Content | str]] | Optional[Content | str]
        ],
    ) -> LlmAgentBuilder: ...
    def after_agent_callback(
        self,
        callback: Callable[
            [CallbackContext], Awaitable[Optional[Content | str]] | Optional[Content | str]
        ],
    ) -> LlmAgentBuilder: ...
    def before_model_callback(
        self,
        callback: Callable[
            [CallbackContext, LlmRequest],
            Awaitable[Optional[BeforeModelResult]] | Optional[BeforeModelResult],
        ],
    ) -> LlmAgentBuilder: ...
    def after_model_callback(
        self,
        callback: Callable[
            [CallbackContext, LlmResponse], Awaitable[Optional[LlmResponse]] | Optional[LlmResponse]
        ],
    ) -> LlmAgentBuilder: ...
    def before_tool_callback(
        self,
        callback: Callable[
            [CallbackContext], Awaitable[Optional[Content | str]] | Optional[Content | str]
        ],
    ) -> LlmAgentBuilder: ...
    def after_tool_callback(
        self,
        callback: Callable[
            [CallbackContext], Awaitable[Optional[Content | str]] | Optional[Content | str]
        ],
    ) -> LlmAgentBuilder: ...
    def build(self) -> LlmAgent: ...

class LlmAgent:
    @staticmethod
    def builder(name: str) -> LlmAgentBuilder: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class CustomAgentBuilder:
    def __init__(self, name: str) -> None: ...
    def description(self, desc: str) -> CustomAgentBuilder: ...
    def handler(
        self, handler: Callable[[InvocationContext], Awaitable[str]]
    ) -> CustomAgentBuilder: ...
    def sub_agent(self, agent: LlmAgent | CustomAgent) -> CustomAgentBuilder: ...
    def build(self) -> CustomAgent: ...

class CustomAgent:
    @staticmethod
    def builder(name: str) -> CustomAgentBuilder: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class SequentialAgent:
    def __init__(self, name: str, agents: List[LlmAgent]) -> None: ...
    @property
    def name(self) -> str: ...

class ParallelAgent:
    def __init__(self, name: str, agents: List[LlmAgent]) -> None: ...
    @property
    def name(self) -> str: ...

class LoopAgent:
    def __init__(self, name: str, agents: List[LlmAgent], max_iterations: int = 10) -> None: ...
    @property
    def name(self) -> str: ...

class ConditionalAgent:
    """Rule-based conditional routing agent.

    Routes execution to one of two agents based on a Python condition function.
    For LLM-based intelligent routing, use LlmConditionalAgent instead.
    """

    def __init__(
        self,
        name: str,
        condition: Callable[[InvocationContext], bool],
        if_agent: Agent,
        else_agent: Optional[Agent] = None,
        description: Optional[str] = None,
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class LlmConditionalAgentBuilder:
    """Builder for LlmConditionalAgent."""

    def description(self, desc: str) -> LlmConditionalAgentBuilder: ...
    def instruction(self, instruction: str) -> LlmConditionalAgentBuilder: ...
    def route(self, label: str, agent: Agent) -> LlmConditionalAgentBuilder: ...
    def default_route(self, agent: Agent) -> LlmConditionalAgentBuilder: ...
    def build(self) -> LlmConditionalAgent: ...

class LlmConditionalAgent:
    """LLM-based intelligent routing agent.

    Uses an LLM to classify user input and route to the appropriate sub-agent
    based on the classification result. Supports multi-way routing.
    """

    @staticmethod
    def builder(
        name: str,
        model: GeminiModel
        | OpenAIModel
        | AnthropicModel
        | DeepSeekModel
        | GroqModel
        | OllamaModel
        | MockLlm,
    ) -> LlmConditionalAgentBuilder: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

# Type alias for agent types
Agent = (
    LlmAgent
    | CustomAgent
    | SequentialAgent
    | ParallelAgent
    | LoopAgent
    | ConditionalAgent
    | LlmConditionalAgent
)

# Tools
class FunctionTool:
    def __init__(
        self,
        name: str,
        description: str,
        handler: Callable[[Any, Dict[str, Any]], Any],
        parameters_schema: Optional[Dict[str, Any]] = None,
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class BasicToolset:
    def __init__(self, name: str) -> None: ...
    def add(self, tool: FunctionTool) -> None: ...
    def tools(self) -> List[FunctionTool]: ...
    @property
    def name(self) -> str: ...
    def __len__(self) -> int: ...

class AgentTool:
    def __init__(
        self,
        agent: LlmAgent | CustomAgent,
        skip_summarization: bool = False,
        forward_artifacts: bool = True,
        timeout_secs: Optional[int] = None,
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class ExitLoopTool:
    def __init__(self) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class LoadArtifactsTool:
    def __init__(self) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

class GoogleSearchTool:
    def __init__(self) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def description(self) -> str: ...

# MCP (Model Context Protocol)
class McpTool:
    """A tool from an MCP server.

    These are returned by McpToolset.get_tools() and can be added to agents
    using LlmAgentBuilder.mcp_tool().
    """

    @property
    def name(self) -> str:
        """Get the tool name."""
        ...
    @property
    def description(self) -> str:
        """Get the tool description."""
        ...
    @property
    def parameters_schema(self) -> Optional[Dict[str, Any]]:
        """Get the tool's JSON schema for parameters."""
        ...

class McpToolset:
    """MCP Toolset - connects to MCP servers and exposes their tools.

    Use factory methods to create instances:
    - McpToolset.from_command(cmd, args) - Stdio transport (subprocess)

    Example:
        mcp = await McpToolset.from_command("npx", ["-y", "@mcp/server-filesystem", "/tmp"])
        tools = await mcp.get_tools()
        agent = LlmAgent.builder("agent").model(model).mcp_tool(tools[0]).build()
        await mcp.close()
    """

    @property
    def name(self) -> str:
        """Get the toolset name."""
        ...
    @staticmethod
    async def from_command(
        command: str,
        args: List[str],
        name: Optional[str] = None,
        tool_filter: Optional[List[str]] = None,
    ) -> McpToolset:
        """Connect to an MCP server via subprocess (stdio transport).

        Args:
            command: The command to run (e.g., "npx", "python", "node")
            args: Arguments to pass to the command
            name: Optional name for this toolset (default: "mcp_toolset")
            tool_filter: Optional list of tool names to include (default: all tools)

        Returns:
            McpToolset instance connected to the MCP server

        Example:
            mcp = await McpToolset.from_command(
                "npx",
                ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
            )
        """
        ...
    async def get_tools(self) -> List[McpTool]:
        """Get the list of tools available from this MCP server.

        Returns:
            List of McpTool objects that can be added to an agent.
        """
        ...
    async def close(self) -> None:
        """Close the MCP connection and shut down the server.

        This should be called when you're done using the toolset.
        """
        ...
    async def is_connected(self) -> bool:
        """Check if the toolset is still connected."""
        ...

# Session
class Session:
    """Session wrapper providing access to session data."""

    @property
    def id(self) -> str:
        """Get the session ID."""
        ...
    @property
    def app_name(self) -> str:
        """Get the application name."""
        ...
    @property
    def user_id(self) -> str:
        """Get the user ID."""
        ...
    @property
    def state(self) -> State:
        """Get the session state."""
        ...
    @property
    def events(self) -> List[Event]:
        """Get all events in the session."""
        ...
    @property
    def last_update_time(self) -> str:
        """Get the last update timestamp as ISO 8601 string."""
        ...
    def event_count(self) -> int:
        """Get the number of events in the session."""
        ...

class InMemorySessionService:
    """In-memory session service with full CRUD operations."""

    def __init__(self) -> None: ...
    async def create(self, request: CreateSessionRequest) -> Session:
        """Create a new session.

        Args:
            request: CreateSessionRequest with app_name, user_id, optional session_id

        Returns:
            Session: The created session
        """
        ...
    async def get(self, request: GetSessionRequest) -> Session:
        """Get an existing session.

        Args:
            request: GetSessionRequest with app_name, user_id, session_id

        Returns:
            Session: The retrieved session

        Raises:
            RuntimeError: If session not found
        """
        ...
    async def list(self, request: ListSessionRequest) -> List[Session]:
        """List all sessions for a user.

        Args:
            request: ListSessionRequest with app_name, user_id

        Returns:
            List[Session]: All sessions for the user
        """
        ...
    async def delete(self, request: DeleteSessionRequest) -> None:
        """Delete a session.

        Args:
            request: DeleteSessionRequest with app_name, user_id, session_id

        Raises:
            RuntimeError: If session not found
        """
        ...

class State:
    def __init__(self) -> None: ...
    def get(self, key: str) -> Any: ...
    def set(self, key: str, value: Any) -> None: ...
    def all(self) -> Dict[str, Any]: ...
    def contains(self, key: str) -> bool: ...
    def remove(self, key: str) -> bool: ...
    def keys(self) -> List[str]: ...
    def __len__(self) -> int: ...

class StreamingMode:
    None_: StreamingMode
    SSE: StreamingMode
    Bidi: StreamingMode

class RunConfig:
    def __init__(self, streaming_mode: StreamingMode = ...) -> None: ...
    @property
    def streaming_mode(self) -> StreamingMode: ...

class CreateSessionRequest:
    def __init__(self, app_name: str, user_id: str, session_id: Optional[str] = None) -> None: ...
    @property
    def app_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def session_id(self) -> Optional[str]: ...
    def with_state(self, key: str, value: Any) -> CreateSessionRequest: ...

class GetSessionRequest:
    def __init__(
        self, app_name: str, user_id: str, session_id: str, num_recent_events: Optional[int] = None
    ) -> None: ...
    @property
    def app_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def session_id(self) -> str: ...
    @property
    def num_recent_events(self) -> Optional[int]: ...

class ListSessionRequest:
    def __init__(self, app_name: str, user_id: str) -> None: ...
    @property
    def app_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...

class DeleteSessionRequest:
    def __init__(self, app_name: str, user_id: str, session_id: str) -> None: ...
    @property
    def app_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def session_id(self) -> str: ...

class GenerateContentConfig:
    temperature: Optional[float]
    top_p: Optional[float]
    top_k: Optional[int]
    max_output_tokens: Optional[int]

    def __init__(
        self,
        temperature: Optional[float] = None,
        top_p: Optional[float] = None,
        top_k: Optional[int] = None,
        max_output_tokens: Optional[int] = None,
        response_schema: Optional[Dict[str, Any]] = None,
    ) -> None: ...
    @property
    def response_schema(self) -> Optional[Dict[str, Any]]: ...

# Runner
class EventStream:
    """Async iterator for streaming events from agent execution.

    Use with `async for`:
    ```python
    async for event in runner.run_stream(user_id, session_id, message):
        if text := event.get_text():
            print(text, end="", flush=True)
    ```
    """

    def __aiter__(self) -> EventStream: ...
    async def __anext__(self) -> Optional[Event]:
        """Get the next event, or None when stream is exhausted."""
        ...

class Runner:
    def __init__(
        self,
        app_name: str,
        agent: LlmAgent,
        session_service: InMemorySessionService,
        run_config: Optional[RunConfig] = None,
    ) -> None: ...
    async def run(self, user_id: str, session_id: str, message: str) -> List[Event]:
        """Run the agent and return all events when complete."""
        ...
    async def run_simple(self, user_id: str, session_id: str, message: str) -> str:
        """Run the agent and return just the final response text."""
        ...
    def run_stream(self, user_id: str, session_id: str, message: str) -> EventStream:
        """Run the agent with streaming - returns an async iterator of events.

        Use with `async for`:
        ```python
        async for event in runner.run_stream(user_id, session_id, message):
            if text := event.get_text():
                print(text, end="", flush=True)
        ```
        """
        ...

async def run_agent(
    agent: LlmAgent,
    message: str,
    user_id: str = "default_user",
    session_id: str = "default_session",
    app_name: str = "adk_app",
) -> str: ...

# Context
class Context:
    @property
    def invocation_id(self) -> str: ...
    @property
    def agent_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def app_name(self) -> str: ...
    @property
    def session_id(self) -> str: ...

class ToolContext:
    @property
    def invocation_id(self) -> str: ...
    @property
    def agent_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def app_name(self) -> str: ...
    @property
    def session_id(self) -> str: ...
    @property
    def function_call_id(self) -> str: ...

class InvocationContext:
    @property
    def invocation_id(self) -> str: ...
    @property
    def agent_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def app_name(self) -> str: ...
    @property
    def session_id(self) -> str: ...
    @property
    def user_content(self) -> Optional[Content]: ...
    @property
    def state(self) -> State: ...

class CallbackContext:
    """Context passed to callback functions."""

    @property
    def invocation_id(self) -> str: ...
    @property
    def agent_name(self) -> str: ...
    @property
    def user_id(self) -> str: ...
    @property
    def app_name(self) -> str: ...
    @property
    def session_id(self) -> str: ...
    @property
    def user_content(self) -> Optional[Content]: ...
    @property
    def state(self) -> State: ...

# Callbacks
class LlmRequest:
    """Request passed to LLM model."""

    model: str
    contents: List[Content]

class LlmResponse:
    """Response from LLM model."""

    partial: bool
    turn_complete: bool

    def __init__(
        self,
        content: Optional[Content] = None,
        partial: bool = False,
        turn_complete: bool = True,
    ) -> None: ...
    @property
    def content(self) -> Optional[Content]: ...
    def get_text(self) -> Optional[str]: ...

class BeforeModelResult:
    """Result from before_model callback to control model execution."""

    @staticmethod
    def cont() -> BeforeModelResult:
        """Continue with the model call."""
        ...
    @staticmethod
    def skip(response_text: str) -> BeforeModelResult:
        """Skip the model call and return the given response."""
        ...

# Error
class AdkError(Exception):
    message: str
    def __init__(self, message: str) -> None: ...

# Guardrails
class Severity:
    """Severity level for guardrail failures."""

    Low: Severity
    Medium: Severity
    High: Severity
    Critical: Severity

class PiiType:
    """Types of PII to detect and redact."""

    Email: PiiType
    Phone: PiiType
    Ssn: PiiType
    CreditCard: PiiType
    IpAddress: PiiType

class ContentFilter:
    """Content filter guardrail for blocking harmful or off-topic content."""

    @staticmethod
    def harmful_content() -> ContentFilter:
        """Create a filter that blocks common harmful content patterns."""
        ...
    @staticmethod
    def on_topic(topic: str, keywords: List[str]) -> ContentFilter:
        """Create a filter that ensures content is on-topic."""
        ...
    @staticmethod
    def max_length(max: int) -> ContentFilter:
        """Create a filter with maximum length."""
        ...
    @staticmethod
    def blocked_keywords(keywords: List[str]) -> ContentFilter:
        """Create a filter with blocked keywords."""
        ...
    @staticmethod
    def custom(
        name: str,
        blocked_keywords: Optional[List[str]] = None,
        required_topics: Optional[List[str]] = None,
        max_length: Optional[int] = None,
        min_length: Optional[int] = None,
        severity: Optional[Severity] = None,
    ) -> ContentFilter:
        """Create a custom content filter with full configuration."""
        ...

class PiiRedactor:
    """PII detection and redaction guardrail."""

    def __init__(self) -> None:
        """Create a new PII redactor with all PII types enabled (Email, Phone, SSN, CreditCard)."""
        ...
    @staticmethod
    def with_types(types: List[PiiType]) -> PiiRedactor:
        """Create a PII redactor with specific types."""
        ...
    def redact(self, text: str) -> tuple[str, List[str]]:
        """Redact PII from text, returns (redacted_text, found_types)."""
        ...

class GuardrailSet:
    """A set of guardrails to run together."""

    def __init__(self) -> None: ...
    def with_content_filter(self, filter: ContentFilter) -> GuardrailSet:
        """Add a content filter to this set."""
        ...
    def with_pii_redactor(self, redactor: PiiRedactor) -> GuardrailSet:
        """Add a PII redactor to this set."""
        ...
    def is_empty(self) -> bool:
        """Check if this set is empty."""
        ...

class GuardrailFailure:
    """A single guardrail failure."""

    name: str
    reason: str
    severity: Severity

class GuardrailResult:
    """Result of running guardrails."""

    passed: bool
    transformed_content: Optional[Content]
    failures: List[GuardrailFailure]

async def run_guardrails(guardrails: GuardrailSet, content: Content) -> GuardrailResult:
    """Run guardrails on content."""
    ...

# Memory
class MemoryEntry:
    """A memory entry with content and metadata."""

    content: Content
    author: str
    timestamp: str

    def __init__(self, content: Content, author: str, timestamp: Optional[str] = None) -> None: ...

class InMemoryMemoryService:
    """In-memory memory service for semantic search."""

    def __init__(self) -> None: ...
    async def add_session(
        self, app_name: str, user_id: str, session_id: str, entries: List[MemoryEntry]
    ) -> None:
        """Add session memories."""
        ...
    async def search(self, app_name: str, user_id: str, query: str) -> List[MemoryEntry]:
        """Search for memories matching a query."""
        ...

# Artifact
class InMemoryArtifactService:
    """In-memory artifact service for binary data storage."""

    def __init__(self) -> None: ...
    async def save(
        self,
        app_name: str,
        user_id: str,
        session_id: str,
        file_name: str,
        data: bytes | str,
        mime_type: Optional[str] = None,
        version: Optional[int] = None,
    ) -> int:
        """Save an artifact (bytes or text). Returns version number."""
        ...
    async def load(
        self,
        app_name: str,
        user_id: str,
        session_id: str,
        file_name: str,
        version: Optional[int] = None,
    ) -> Part:
        """Load an artifact. Returns Part containing the data."""
        ...
    async def delete(
        self,
        app_name: str,
        user_id: str,
        session_id: str,
        file_name: str,
        version: Optional[int] = None,
    ) -> None:
        """Delete an artifact."""
        ...
    async def list(self, app_name: str, user_id: str, session_id: str) -> List[str]:
        """List all artifact names in a session."""
        ...
    async def versions(
        self, app_name: str, user_id: str, session_id: str, file_name: str
    ) -> List[int]:
        """Get all versions of an artifact."""
        ...

__version__: str
