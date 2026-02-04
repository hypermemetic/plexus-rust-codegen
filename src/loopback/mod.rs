//! Module for loopback namespace
//! Do not edit manually

use crate::client::PlexusClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

// === Types ===

/// Status of an approval request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "timed_out")]
    TimedOut,
}

/// A pending approval request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub created_at: i64,
    pub id: String,
    pub input: serde_json::Value,
    pub resolved_at: Option<i64>,
    pub response_message: Option<String>,
    pub session_id: String,
    pub status: ApprovalStatus,
    pub tool_name: String,
    pub tool_use_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConfigureResult {
    Ok {
mcp_config: serde_json::Value,
    },
    Error {
message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RespondResult {
    Ok {
approval_id: String,
    },
    Error {
message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PendingResult {
    Ok {
approvals: Vec<ApprovalRequest>,
    },
    Error {
message: String,
    },
}

// === Methods ===

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("loopback.schema", serde_json::Value::Null).await
}

/// Generate MCP configuration for a loopback session
pub async fn configure(client: &PlexusClient, session_id: String) -> Result<ConfigureResult> {
    client.call_single("loopback.configure", json!({ "session_id": session_id })).await
}

/// Permission prompt handler - blocks until parent approves/denies  This is called by Claude Code CLI via --permission-prompt-tool. It blocks (polls) until the parent calls loopback.respond().  Returns a JSON string (not object) because Claude Code expects the MCP response to have the permission JSON already stringified in content[0].text. See: https://github.com/anthropics/claude-code/blob/main/docs/permission-prompt-tool.md
pub async fn permit(client: &PlexusClient, input: serde_json::Value, tool_name: String, tool_use_id: String) -> Result<serde_json::Value> {
    client.call_single("loopback.permit", json!({ "input": input, "tool_name": tool_name, "tool_use_id": tool_use_id })).await
}

/// List pending approval requests
pub async fn pending(client: &PlexusClient, session_id: Option<String>) -> Result<PendingResult> {
    client.call_single("loopback.pending", json!({ "session_id": session_id })).await
}

/// Respond to a pending approval request
pub async fn respond(client: &PlexusClient, approval_id: String, approve: bool, message: Option<String>) -> Result<RespondResult> {
    client.call_single("loopback.respond", json!({ "approval_id": approval_id, "approve": approve, "message": message })).await
}
