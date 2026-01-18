# Plan: Expose Missing adk-rust Implementations to Python

> **Status:** Phases 1-3 COMPLETED. Phase 4 pending.

## Overview

This plan adds Python bindings for adk-rust features needed for complete agent development, especially what's required by Studio. The implementation follows existing PyO3 patterns in the codebase.

---

## Current State (After Phase 3)

**Now Exposed (45+ classes):**
- Types: `Content`, `Part`, `Event`
- Models: `GeminiModel`, `OpenAIModel`, `AnthropicModel`, `DeepSeekModel`, `GroqModel`, `OllamaModel`, `MockLlm`
- Agents: `LlmAgent`, `LlmAgentBuilder`, `CustomAgent`, `CustomAgentBuilder`, `SequentialAgent`, `ParallelAgent`, `LoopAgent`, `ConditionalAgent`, `LlmConditionalAgent`, `LlmConditionalAgentBuilder`
- Tools: `FunctionTool`, `BasicToolset`, `AgentTool`, `ExitLoopTool`, `GoogleSearchTool`, `LoadArtifactsTool`
- Session: `InMemorySessionService`, `State`, `RunConfig`, `CreateSessionRequest`, `GetSessionRequest`, `StreamingMode`, `GenerateContentConfig`
- Runner: `Runner`, `run_agent()`
- Context: `Context`, `ToolContext`, `InvocationContext`, `CallbackContext`
- Callbacks: `LlmRequest`, `LlmResponse`, `BeforeModelResult`
- Guardrails: `Severity`, `PiiType`, `ContentFilter`, `PiiRedactor`, `GuardrailSet`, `GuardrailResult`, `GuardrailFailure`, `run_guardrails()`
- Memory: `MemoryEntry`, `InMemoryMemoryService`
- Artifacts: `InMemoryArtifactService`
- Error: `AdkError`

---

## Completed Implementation

### Phase 1: Core Agent Capabilities (DONE)

#### 1.1 CustomAgent (Functional)
**Files:** `src/agent/custom.rs`

Python functions as agent handlers:
```python
async def my_handler(ctx: InvocationContext) -> str:
    return f"Processed: {ctx.user_content.get_text()}"

agent = (CustomAgent.builder("my_agent")
    .handler(my_handler)
    .description("Custom logic agent")
    .build())
```

#### 1.2 GenerateContentConfig
**Files:** `src/session/mod.rs`

Model generation parameters:
```python
config = GenerateContentConfig(temperature=0.7, max_output_tokens=1024)
```

#### 1.3 AgentTool
**Files:** `src/tool/agent_tool.rs`

Use agents as tools for composition:
```python
math_agent = LlmAgent.builder("math").model(model).build()
math_tool = AgentTool(math_agent)
coordinator = LlmAgent.builder("main").model(model).tool(math_tool).build()
```

---

### Phase 2: Routing and Flow Control (DONE)

#### 2.1 ConditionalAgent
**Files:** `src/agent/conditional.rs`

Rule-based routing:
```python
router = ConditionalAgent(
    "router",
    condition=lambda ctx: ctx.state.get("premium"),
    if_agent=premium_agent,
    else_agent=basic_agent
)
```

#### 2.2 LlmConditionalAgent
LLM-powered multi-route routing:
```python
router = (LlmConditionalAgent.builder("router", model)
    .instruction("Classify as 'technical', 'billing', or 'general'")
    .route("technical", tech_agent)
    .route("billing", billing_agent)
    .default_route(general_agent)
    .build())
```

#### 2.3 Callbacks
**Files:** `src/callbacks.rs`, `src/agent/llm.rs`

Intercept agent/model/tool execution:
```python
def before_model(ctx, request):
    print(f"Calling model with: {request}")
    return BeforeModelResult.cont()  # or .skip(response_text)

agent = (LlmAgent.builder("agent")
    .model(model)
    .before_model_callback(before_model)
    .after_model_callback(after_model)
    .before_tool_callback(before_tool)
    .after_tool_callback(after_tool)
    .build())
```

---

### Phase 3: Safety and Persistence (DONE)

#### 3.1 Guardrails
**Files:** `src/guardrail/mod.rs`

Content filtering and PII protection:
```python
# Content filtering
harmful = ContentFilter.harmful_content()
on_topic = ContentFilter.on_topic("cooking", ["recipe", "cook", "bake"])
max_len = ContentFilter.max_length(1000)

# PII redaction
pii = PiiRedactor()  # All types
pii_email_only = PiiRedactor.with_types([PiiType.Email])

# Combine and run
guardrails = GuardrailSet().with_content_filter(harmful).with_pii_redactor(pii)
result = await run_guardrails(guardrails, content)
if not result.passed:
    print(result.failures)
```

#### 3.2 Memory System
**Files:** `src/memory.rs`

Semantic memory for long-term context:
```python
memory = InMemoryMemoryService()
await memory.add_session(app_name, user_id, session_id, [
    MemoryEntry(Content.user("Important fact"), "user")
])
results = await memory.search(app_name, user_id, "fact")
```

#### 3.3 Artifact System
**Files:** `src/artifact.rs`

Binary data storage (images, files):
```python
artifacts = InMemoryArtifactService()
version = await artifacts.save(app_name, user_id, session_id, "chart.png", image_bytes)
part = await artifacts.load(app_name, user_id, session_id, "chart.png")
files = await artifacts.list(app_name, user_id, session_id)
versions = await artifacts.versions(app_name, user_id, session_id, "chart.png")
await artifacts.delete(app_name, user_id, session_id, "chart.png")
```

---

## Remaining Work

### Phase 4: Advanced Features (TODO)

#### 4.1 McpToolset
**Files:** `src/tool/mcp.rs` (new)

MCP (Model Context Protocol) integration:
```python
mcp = await McpToolset.from_command("npx", ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"])
agent = LlmAgent.builder("agent").model(model).toolset(mcp).build()
```

#### 4.2 Event Streaming
**Files:** modify `src/runner/mod.rs`

True async iteration over events:
```python
async for event in runner.run_stream(user_id, session_id, message):
    print(event.get_text())
```

---

## Files Created/Modified

| File | Action | Phase |
|------|--------|-------|
| `src/agent/custom.rs` | Rewritten | 1 |
| `src/agent/conditional.rs` | Created | 2 |
| `src/tool/agent_tool.rs` | Created | 1 |
| `src/callbacks.rs` | Created | 2 |
| `src/guardrail/mod.rs` | Created | 3 |
| `src/memory.rs` | Created | 3 |
| `src/artifact.rs` | Created | 3 |
| `src/context.rs` | Modified (PyInvocationContext, PyCallbackContext) | 1-2 |
| `src/session/mod.rs` | Modified (GenerateContentConfig) | 1 |
| `src/agent/llm.rs` | Modified (callbacks) | 2 |
| `src/agent/mod.rs` | Modified (exports) | 1-2 |
| `src/tool/mod.rs` | Modified (exports) | 1 |
| `src/lib.rs` | Modified (register classes) | All |
| `python/adk_rust/__init__.py` | Modified (exports) | All |
| `python/adk_rust/__init__.pyi` | Modified (type stubs) | All |
| `Cargo.toml` | Modified (adk-guardrail) | 3 |

---

## Priority Summary

| Priority | Features | Status |
|----------|----------|--------|
| **P0** | CustomAgent, AgentTool, ConditionalAgent, Callbacks | DONE |
| **P1** | Guardrails, Memory, Artifacts, GenerateContentConfig | DONE |
| **P2** | McpToolset, Event Streaming | TODO |
| **P3** | Browser, Graph workflows, Schema serialization | Future |
