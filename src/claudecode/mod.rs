//! Module for claudecode namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

use crate::cone::Position;

// === Types ===

/// Model selection for Claude Code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "sonnet")]
    Sonnet,
    #[serde(rename = "haiku")]
    Haiku,
}

/// Result of deleting a session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeleteResult {
    Deleted {
id: String,
    },
    Error {
message: String,
    },
}

/// Information about an active stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// When the stream ended (if complete/failed)
    pub ended_at: Option<i64>,
    /// Error message if failed
    pub error: Option<String>,
    /// Number of events buffered
    pub event_count: u64,
    /// Read position (how many events have been consumed)
    pub read_position: u64,
    /// Session this stream belongs to
    pub session_id: String,
    /// When the stream started
    pub started_at: i64,
    /// Current status
    pub status: StreamStatus,
    /// Unique stream identifier
    pub stream_id: String,
    /// Position of the user message node (set at start)
    pub user_position: Option<Position>,
}

/// ClaudeCode session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeConfig {
    /// Claude Code's internal session ID (for --resume)
    pub claude_session_id: Option<String>,
    /// Created timestamp
    pub created_at: i64,
    /// The canonical head - current position in conversation tree
    pub head: Position,
    /// Unique identifier for this session
    pub id: String,
    /// Enable loopback mode - routes tool permissions through parent for approval
    pub loopback_enabled: bool,
    /// MCP server configuration (JSON)
    pub mcp_config: serde_json::Value,
    /// Additional metadata
    pub metadata: serde_json::Value,
    /// Model to use
    pub model: Model,
    /// Human-readable name
    pub name: String,
    /// System prompt / instructions
    pub system_prompt: Option<String>,
    /// Last updated timestamp
    pub updated_at: i64,
    /// Working directory for Claude Code
    pub working_dir: String,
}

/// Lightweight session info (for listing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeCodeInfo {
    pub claude_session_id: Option<String>,
    pub created_at: i64,
    pub head: Position,
    pub id: String,
    pub loopback_enabled: bool,
    pub model: Model,
    pub name: String,
    pub working_dir: String,
}

/// Result of polling a stream for events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PollResult {
    Ok {
        /// Events since last poll (or from specified offset)
events: Vec<BufferedEvent>,
        /// True if there are more events available
has_more: bool,
        /// Current read position after this poll
read_position: u64,
        /// Current stream status
status: StreamStatus,
        /// Total events in buffer
total_events: u64,
    },
    Error {
message: String,
    },
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatUsage {
    pub cost_usd: Option<f64>,
    pub input_tokens: Option<u64>,
    pub num_turns: Option<i64>,
    pub output_tokens: Option<u64>,
}

/// Result of listing sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ListResult {
    Ok {
sessions: Vec<ClaudeCodeInfo>,
    },
    Error {
message: String,
    },
}

/// Result of getting a session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum GetResult {
    Ok {
config: ClaudeCodeConfig,
    },
    Error {
message: String,
    },
}

/// A buffered event in the stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferedEvent {
    /// The chat event
    pub event: ChatEvent,
    /// Sequence number within the stream
    pub seq: u64,
    /// Timestamp when event was received
    pub timestamp: i64,
}

/// Result of starting an async chat (non-blocking)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChatStartResult {
    Started {
session_id: String,
stream_id: String,
    },
    Error {
message: String,
    },
}

/// Result of listing active streams
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum StreamListResult {
    Ok {
streams: Vec<StreamInfo>,
    },
    Error {
message: String,
    },
}

/// Events emitted during chat streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChatEvent {
    /// Chat started - user message stored, streaming begins
    Start {
id: String,
user_position: Position,
    },
    /// Content chunk (streaming tokens)
    Content {
text: String,
    },
    /// Thinking block - Claude's internal reasoning
    Thinking {
thinking: String,
    },
    /// Tool use detected
    ToolUse {
input: serde_json::Value,
tool_name: String,
tool_use_id: String,
    },
    /// Tool result received
    ToolResult {
is_error: bool,
output: String,
tool_use_id: String,
    },
    /// Chat complete - response stored, head updated
    Complete {
claude_session_id: String,
new_head: Position,
usage: Option<ChatUsage>,
    },
    /// Passthrough for unrecognized Claude Code events
    /// Data is stored separately (referenced by handle) and also forwarded inline
    Passthrough {
data: serde_json::Value,
event_type: String,
handle: String,
    },
    /// Error during chat
    Error {
message: String,
    },
}

/// Result of forking a session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ForkResult {
    Forked {
head: Position,
id: String,
    },
    Error {
message: String,
    },
}

/// Result of creating a session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CreateResult {
    Created {
head: Position,
id: String,
    },
    Error {
message: String,
    },
}

/// Status of an active stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "awaiting_permission")]
    AwaitingPermission,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
}

// === Methods ===

/// Fork a session to create a branch point
pub async fn fork(client: &PlexusClient, name: String, new_name: String) -> Result<ForkResult> {
    client.call_single("claudecode.fork", json!({ "name": name, "new_name": new_name })).await
}

/// Create a new Claude Code session
pub async fn create(client: &PlexusClient, loopback_enabled: Option<bool>, model: Model, name: String, system_prompt: Option<String>, working_dir: String) -> Result<CreateResult> {
    client.call_single("claudecode.create", json!({ "loopback_enabled": loopback_enabled, "model": model, "name": name, "system_prompt": system_prompt, "working_dir": working_dir })).await
}

/// Delete a session
pub async fn delete(client: &PlexusClient, name: String) -> Result<DeleteResult> {
    client.call_single("claudecode.delete", json!({ "name": name })).await
}

/// Poll a stream for new events  Returns events since the last poll (or from the specified offset). Use this to read events from an async chat started with chat_async.
pub async fn poll(client: &PlexusClient, from_seq: Option<u64>, limit: Option<u64>, stream_id: String) -> Result<PollResult> {
    client.call_single("claudecode.poll", json!({ "from_seq": from_seq, "limit": limit, "stream_id": stream_id })).await
}

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("claudecode.schema", serde_json::Value::Null).await
}

/// List active streams  Returns all active streams, optionally filtered by session.
pub async fn streams(client: &PlexusClient, session_id: Option<String>) -> Result<StreamListResult> {
    client.call_single("claudecode.streams", json!({ "session_id": session_id })).await
}

/// Get session configuration details
pub async fn get(client: &PlexusClient, name: String) -> Result<GetResult> {
    client.call_single("claudecode.get", json!({ "name": name })).await
}

/// Start an async chat - returns immediately with stream_id for polling  This is the non-blocking version of chat, designed for loopback scenarios where the parent needs to poll for events and handle tool approvals.
pub async fn chat_async(client: &PlexusClient, ephemeral: Option<bool>, name: String, prompt: String) -> Result<ChatStartResult> {
    client.call_single("claudecode.chat_async", json!({ "ephemeral": ephemeral, "name": name, "prompt": prompt })).await
}

/// Chat with a session, streaming tokens like Cone
pub async fn chat(client: &PlexusClient, ephemeral: Option<bool>, name: String, prompt: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChatEvent>> + Send>>> {
    let stream = client.call_stream("claudecode.chat", json!({ "ephemeral": ephemeral, "name": name, "prompt": prompt })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChatEvent>(content) {
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

/// List all Claude Code sessions
pub async fn list(client: &PlexusClient) -> Result<ListResult> {
    client.call_single("claudecode.list", serde_json::Value::Null).await
}
