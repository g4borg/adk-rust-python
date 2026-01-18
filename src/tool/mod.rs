//! Tool bindings for Python
//!
//! This module provides Python bindings for ADK tools:
//! - `FunctionTool` - Wrap Python functions as tools
//! - `BasicToolset` - Collection of tools
//! - `AgentTool` - Use agents as tools
//! - `ExitLoopTool` - Exit loop agents
//! - `LoadArtifactsTool` - Load artifacts into context
//! - `GoogleSearchTool` - Google search (Gemini grounding)
//! - `McpToolset` - MCP (Model Context Protocol) integration

mod agent_tool;
mod builtin;
pub mod function;
mod mcp;

pub use agent_tool::PyAgentTool;
pub use builtin::{PyExitLoopTool, PyGoogleSearchTool, PyLoadArtifactsTool};
pub use function::{PyBasicToolset, PyFunctionTool, PyMcpToolWrapper};
pub use mcp::PyMcpToolset;
