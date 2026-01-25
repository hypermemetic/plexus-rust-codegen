//! Module for jsexec namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// === Types ===

/// Location within a script file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Column number (1-indexed)
    pub column: i64,
    /// Filename if available
    pub filename: Option<String>,
    /// Line number (1-indexed)
    pub line: i64,
}

/// Log level for console output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}

/// Unique identifier for a stored script
pub type ScriptId = String;

/// Unique identifier for a single execution
pub type ExecutionId = String;

/// A single frame in a stack trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    /// Function name if available
    pub function_name: Option<String>,
    /// Source location of this frame
    pub location: SourceLocation,
}

/// Events emitted during JavaScript execution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum JsExecEvent {
    /// Execution has started
    ExecutionStarted {
execution_id: ExecutionId,
script_id: Option<ScriptId>,
worker_id: String,
    },
    /// Execution has completed successfully
    ExecutionCompleted {
execution_id: ExecutionId,
metrics: ResourceMetrics,
    },
    /// Console output from the script
    Console {
args: Vec<serde_json::Value>,
level: LogLevel,
timestamp_ms: u64,
    },
    /// Script returned a value
    Returned {
value: serde_json::Value,
    },
    /// A fetch request has started
    FetchStarted {
method: String,
request_id: String,
url: String,
    },
    /// A fetch request has completed
    FetchCompleted {
duration_ms: u64,
request_id: String,
status: i64,
    },
    /// An error occurred during execution
    Error {
location: Option<SourceLocation>,
message: String,
name: String,
stack: Vec<StackFrame>,
    },
    /// Resource usage is approaching limits
    ResourceWarning {
current: u64,
limit: u64,
message: String,
resource: String,
    },
    /// A script has been stored
    ScriptStored {
hash: String,
name: String,
script_id: ScriptId,
size_bytes: u64,
    },
    /// A script has been deleted
    ScriptDeleted {
script_id: ScriptId,
    },
}

/// Metrics collected during script execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU time consumed in milliseconds
    pub cpu_time_ms: u64,
    /// Peak memory usage in bytes
    pub memory_peak_bytes: u64,
    /// Current memory usage in bytes
    pub memory_used_bytes: u64,
    /// Wall clock time elapsed in milliseconds
    pub wall_time_ms: u64,
}

// === Methods ===

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("jsexec.schema", serde_json::Value::Null).await
}

/// Delete a stored script
pub async fn delete_script(client: &PlexusClient, script_id: ScriptId) -> Result<Pin<Box<dyn Stream<Item = Result<JsExecEvent>> + Send>>> {
    let stream = client.call_stream("jsexec.delete_script", json!({ "script_id": script_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<JsExecEvent>(content) {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            Ok(PlexusStreamItem::Error { message, code, .. }) => {
                Some(Err(anyhow!("Plexus error{}: {}",
                    code.map(|c| format!(" [{}]", c)).unwrap_or_default(),
                    message
                )))
            }
            Ok(PlexusStreamItem::Progress { .. }) => None, // Skip progress
            Ok(PlexusStreamItem::Done { .. }) => None, // Stream will end
            Err(e) => Some(Err(e)),
        }
    });

    Ok(Box::pin(typed_stream))
}

/// Execute JavaScript code with additional modules loaded by path
pub async fn execute_with_modules(client: &PlexusClient, code: String, module_paths: Vec<String>) -> Result<Pin<Box<dyn Stream<Item = Result<JsExecEvent>> + Send>>> {
    let stream = client.call_stream("jsexec.execute_with_modules", json!({ "code": code, "module_paths": module_paths })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<JsExecEvent>(content) {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            Ok(PlexusStreamItem::Error { message, code, .. }) => {
                Some(Err(anyhow!("Plexus error{}: {}",
                    code.map(|c| format!(" [{}]", c)).unwrap_or_default(),
                    message
                )))
            }
            Ok(PlexusStreamItem::Progress { .. }) => None, // Skip progress
            Ok(PlexusStreamItem::Done { .. }) => None, // Stream will end
            Err(e) => Some(Err(e)),
        }
    });

    Ok(Box::pin(typed_stream))
}

/// List all stored scripts
pub async fn list_scripts(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("jsexec.list_scripts", serde_json::Value::Null).await
}

/// Evaluate a JavaScript expression and return the result
pub async fn eval(client: &PlexusClient, expr: String) -> Result<Pin<Box<dyn Stream<Item = Result<JsExecEvent>> + Send>>> {
    let stream = client.call_stream("jsexec.eval", json!({ "expr": expr })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<JsExecEvent>(content) {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            Ok(PlexusStreamItem::Error { message, code, .. }) => {
                Some(Err(anyhow!("Plexus error{}: {}",
                    code.map(|c| format!(" [{}]", c)).unwrap_or_default(),
                    message
                )))
            }
            Ok(PlexusStreamItem::Progress { .. }) => None, // Skip progress
            Ok(PlexusStreamItem::Done { .. }) => None, // Stream will end
            Err(e) => Some(Err(e)),
        }
    });

    Ok(Box::pin(typed_stream))
}

/// Run a previously stored script
pub async fn execute_script(client: &PlexusClient, script_id: ScriptId) -> Result<Pin<Box<dyn Stream<Item = Result<JsExecEvent>> + Send>>> {
    let stream = client.call_stream("jsexec.execute_script", json!({ "script_id": script_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<JsExecEvent>(content) {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            Ok(PlexusStreamItem::Error { message, code, .. }) => {
                Some(Err(anyhow!("Plexus error{}: {}",
                    code.map(|c| format!(" [{}]", c)).unwrap_or_default(),
                    message
                )))
            }
            Ok(PlexusStreamItem::Progress { .. }) => None, // Skip progress
            Ok(PlexusStreamItem::Done { .. }) => None, // Stream will end
            Err(e) => Some(Err(e)),
        }
    });

    Ok(Box::pin(typed_stream))
}

/// Execute JavaScript code and stream results
pub async fn execute(client: &PlexusClient, code: String) -> Result<Pin<Box<dyn Stream<Item = Result<JsExecEvent>> + Send>>> {
    let stream = client.call_stream("jsexec.execute", json!({ "code": code })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<JsExecEvent>(content) {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            Ok(PlexusStreamItem::Error { message, code, .. }) => {
                Some(Err(anyhow!("Plexus error{}: {}",
                    code.map(|c| format!(" [{}]", c)).unwrap_or_default(),
                    message
                )))
            }
            Ok(PlexusStreamItem::Progress { .. }) => None, // Skip progress
            Ok(PlexusStreamItem::Done { .. }) => None, // Stream will end
            Err(e) => Some(Err(e)),
        }
    });

    Ok(Box::pin(typed_stream))
}

/// Store a script for later execution
pub async fn store(client: &PlexusClient, code: String, description: Option<String>, name: String) -> Result<Pin<Box<dyn Stream<Item = Result<JsExecEvent>> + Send>>> {
    let stream = client.call_stream("jsexec.store", json!({ "code": code, "description": description, "name": name })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<JsExecEvent>(content) {
                    Ok(data) => Some(Ok(data)),
                    Err(e) => Some(Err(e.into())),
                }
            }
            Ok(PlexusStreamItem::Error { message, code, .. }) => {
                Some(Err(anyhow!("Plexus error{}: {}",
                    code.map(|c| format!(" [{}]", c)).unwrap_or_default(),
                    message
                )))
            }
            Ok(PlexusStreamItem::Progress { .. }) => None, // Skip progress
            Ok(PlexusStreamItem::Done { .. }) => None, // Stream will end
            Err(e) => Some(Err(e)),
        }
    });

    Ok(Box::pin(typed_stream))
}
