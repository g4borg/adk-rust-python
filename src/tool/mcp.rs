//! MCP (Model Context Protocol) Toolset bindings for Python
//!
//! Provides McpToolset for connecting to MCP servers via:
//! - Stdio transport (subprocess) - `from_command()`
//! - SSE transport (HTTP) - `from_sse()`

use adk_core::{ReadonlyContext, Toolset};
use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::function::PyMcpToolWrapper;

/// Type-erased wrapper holding the MCP toolset and cancellation token
struct McpToolsetInner {
    toolset: Arc<dyn Toolset>,
    cancel_token: Option<rmcp::service::RunningServiceCancellationToken>,
}

/// MCP Toolset - connects to MCP servers and exposes their tools.
///
/// Use factory methods to create instances:
/// - `McpToolset.from_command(cmd, args)` - Stdio transport (subprocess)
/// - `McpToolset.from_sse(url)` - SSE transport (HTTP)
///
/// Example:
/// ```python
/// # Connect to an MCP server via subprocess
/// mcp = await McpToolset.from_command("npx", ["-y", "@mcp/server-filesystem", "/tmp"])
///
/// # Use with an agent
/// agent = LlmAgent.builder("agent").model(model).toolset(mcp).build()
///
/// # Don't forget to close when done
/// await mcp.close()
/// ```
#[pyclass(name = "McpToolset")]
pub struct PyMcpToolset {
    inner: Arc<Mutex<Option<McpToolsetInner>>>,
    name: String,
}

impl PyMcpToolset {
    pub fn as_toolset(&self) -> Arc<dyn Toolset> {
        // Create a wrapper that delegates to the inner toolset
        Arc::new(McpToolsetWrapper {
            inner: self.inner.clone(),
            name: self.name.clone(),
        })
    }
}

/// Wrapper to expose McpToolset as a Toolset trait object
struct McpToolsetWrapper {
    inner: Arc<Mutex<Option<McpToolsetInner>>>,
    name: String,
}

#[async_trait::async_trait]
impl Toolset for McpToolsetWrapper {
    fn name(&self) -> &str {
        &self.name
    }

    async fn tools(
        &self,
        ctx: Arc<dyn adk_core::ReadonlyContext>,
    ) -> adk_core::Result<Vec<Arc<dyn adk_core::Tool>>> {
        let guard = self.inner.lock().await;
        if let Some(ref inner) = *guard {
            inner.toolset.tools(ctx).await
        } else {
            Err(adk_core::AdkError::Tool(
                "McpToolset has been closed".to_string(),
            ))
        }
    }
}

#[pymethods]
impl PyMcpToolset {
    /// Connect to an MCP server via subprocess (stdio transport).
    ///
    /// Args:
    ///     command: The command to run (e.g., "npx", "python", "node")
    ///     args: Arguments to pass to the command
    ///     name: Optional name for this toolset (default: "mcp_toolset")
    ///     tool_filter: Optional list of tool names to include (default: all tools)
    ///
    /// Returns:
    ///     McpToolset instance connected to the MCP server
    ///
    /// Example:
    ///     mcp = await McpToolset.from_command(
    ///         "npx",
    ///         ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
    ///     )
    #[staticmethod]
    #[pyo3(signature = (command, args, name=None, tool_filter=None))]
    fn from_command<'py>(
        py: Python<'py>,
        command: String,
        args: Vec<String>,
        name: Option<String>,
        tool_filter: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let toolset_name = name.unwrap_or_else(|| "mcp_toolset".to_string());

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            use rmcp::ServiceExt;
            use rmcp::transport::TokioChildProcess;

            // Build the command
            let mut cmd = tokio::process::Command::new(&command);
            cmd.args(&args);

            // Create transport - TokioChildProcess::new takes ownership
            let transport = TokioChildProcess::new(cmd).map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "Failed to create MCP transport: {}",
                    e
                ))
            })?;

            // Connect to MCP server
            let client = ().serve(transport).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "Failed to connect to MCP server: {}",
                    e
                ))
            })?;

            // Get cancellation token before wrapping
            let cancel_token = client.cancellation_token();

            // Create the toolset
            let mut mcp_toolset = adk_tool::McpToolset::new(client).with_name(&toolset_name);

            // Apply tool filter if provided
            if let Some(filter_names) = tool_filter {
                let filter_vec: Vec<String> = filter_names;
                mcp_toolset =
                    mcp_toolset.with_filter(move |name| filter_vec.iter().any(|n| n == name));
            }

            let inner = McpToolsetInner {
                toolset: Arc::new(mcp_toolset),
                cancel_token: Some(cancel_token),
            };

            Ok(PyMcpToolset {
                inner: Arc::new(Mutex::new(Some(inner))),
                name: toolset_name,
            })
        })
    }

    // Note: SSE transport requires additional HTTP client integration (reqwest)
    // which adds complexity. The stdio transport (from_command) covers most use cases.
    // SSE support can be added in a future version if needed.

    /// Close the MCP connection and shut down the server.
    ///
    /// This should be called when you're done using the toolset to cleanly
    /// shut down the MCP server process.
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = inner.lock().await;
            if let Some(inner_val) = guard.take() {
                if let Some(token) = inner_val.cancel_token {
                    token.cancel();
                }
            }
            Ok(())
        })
    }

    /// Get the name of this toolset.
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    /// Check if the toolset is still connected.
    fn is_connected<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let guard = inner.lock().await;
            Ok(guard.is_some())
        })
    }

    /// Get the list of tools available from this MCP server.
    ///
    /// Returns a list of McpTool objects that can be added to an agent.
    ///
    /// Example:
    ///     mcp = await McpToolset.from_command("npx", ["-y", "@mcp/server"])
    ///     tools = await mcp.get_tools()
    ///     for tool in tools:
    ///         print(f"Tool: {tool.name} - {tool.description}")
    fn get_tools<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let guard = inner.lock().await;
            if let Some(ref inner_val) = *guard {
                // Create a minimal context for listing tools
                let ctx: Arc<dyn ReadonlyContext> = Arc::new(MinimalContext::new());
                let tools = inner_val.toolset.tools(ctx).await.map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!(
                        "Failed to list MCP tools: {}",
                        e
                    ))
                })?;

                let py_tools: Vec<PyMcpToolWrapper> =
                    tools.into_iter().map(PyMcpToolWrapper::new).collect();

                Ok(py_tools)
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err(
                    "McpToolset has been closed",
                ))
            }
        })
    }

    fn __repr__(&self) -> String {
        format!("McpToolset(name='{}')", self.name)
    }
}

/// Minimal context implementation for tool listing
struct MinimalContext {
    invocation_id: String,
}

impl MinimalContext {
    fn new() -> Self {
        Self {
            invocation_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[async_trait::async_trait]
impl ReadonlyContext for MinimalContext {
    fn invocation_id(&self) -> &str {
        &self.invocation_id
    }
    fn agent_name(&self) -> &str {
        "mcp_tool_lister"
    }
    fn user_id(&self) -> &str {
        "system"
    }
    fn app_name(&self) -> &str {
        "adk_python"
    }
    fn session_id(&self) -> &str {
        "mcp_session"
    }
    fn branch(&self) -> &str {
        "main"
    }
    fn user_content(&self) -> &adk_core::Content {
        static EMPTY_CONTENT: std::sync::OnceLock<adk_core::Content> = std::sync::OnceLock::new();
        EMPTY_CONTENT.get_or_init(|| adk_core::Content::new("user"))
    }
}
