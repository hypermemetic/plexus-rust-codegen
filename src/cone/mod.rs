//! Module for cone namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// === Types ===

/// Identifier for a cone - either by name or UUID
/// 
/// CLI usage: Just pass the name or UUID directly (e.g., "my-assistant" or "550e8400-...")
/// The CLI/API will handle the conversion to the appropriate lookup type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConeIdentifier {
    /// Lookup cone by its human-readable name
    ByName {
        /// Cone name (supports partial matching, e.g., "assistant" or "assistant#550e")
name: String,
    },
    /// Lookup cone by its UUID
    ById {
        /// Cone UUID
id: String,
    },
}

pub type UUID = String;

/// Result of cone.create
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CreateResult {
    ConeCreated {
cone_id: String,
        /// Initial position (tree + root node)
head: Position,
    },
    Error {
message: String,
    },
}

/// Verification status for a model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "unverified")]
    Unverified,
    #[serde(rename = "broken")]
    Broken,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "not_found")]
    NotFound,
}

/// Message builder format - determines how messages are structured for the API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessageFormat {

}

/// Events emitted during cone.chat (streaming)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChatEvent {
    /// Chat response started
    ChatStart {
cone_id: String,
        /// Position of the user message node
user_position: Position,
    },
    /// Chat content chunk (streaming)
    ChatContent {
cone_id: String,
content: String,
    },
    /// Chat response complete
    ChatComplete {
cone_id: String,
        /// The new head position (tree + response node)
new_head: Position,
        /// Total tokens used (if available)
usage: Option<ChatUsage>,
    },
    Error {
message: String,
    },
}

/// Result of cone.delete
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeleteResult {
    ConeDeleted {
cone_id: String,
    },
    Error {
message: String,
    },
}

/// Result of cone.get
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum GetResult {
    ConeData {
cone: ConeConfig,
    },
    Error {
message: String,
    },
}

/// Result of cone.registry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RegistryResult {
    Registry,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

/// Cone configuration - defines an cone's identity and behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConeConfig {
    /// Created timestamp
    pub created_at: i64,
    /// The canonical head - current position in conversation tree
    /// This couples tree_id and node_id together
    pub head: Position,
    /// Unique identifier for this cone
    pub id: String,
    /// Additional configuration metadata
    pub metadata: serde_json::Value,
    /// Model ID to use (e.g., "gpt-4o-mini", "claude-3-haiku-20240307")
    pub model_id: String,
    /// Human-readable name
    pub name: String,
    /// System prompt / instructions for the cone
    pub system_prompt: Option<String>,
    /// Last updated timestamp
    pub updated_at: i64,
}

/// Result of cone.list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ListResult {
    ConeList {
cones: Vec<ConeInfo>,
    },
    Error {
message: String,
    },
}

/// Serializable model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelExport {
    /// Model capabilities
    pub capabilities: Capabilities,
    /// Model constraints
    pub constraints: Constraints,
    /// Model family (e.g., "claude", "gpt")
    pub family: String,
    /// Model identifier
    pub id: String,
    /// Lab/organization that created the model
    pub lab: Option<String>,
    /// Human-readable name
    pub name: String,
    /// Pricing information
    pub pricing: Pricing,
    /// Service this model uses
    pub service: String,
    /// Verification status
    pub status: VerificationStatus,
    /// Use cases this model is suited for
    pub use_cases: Vec<String>,
    /// Variant (e.g., "haiku", "sonnet", "opus")
    pub variant: Option<String>,
    /// Version string if available
    pub version: Option<String>,
}

/// Serializable service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceExport {
    /// Base URL for the API
    pub base_url: String,
    /// Message format type
    pub message_format: MessageFormat,
    /// Number of models using this service
    pub model_count: i64,
    /// Service identifier (e.g., "openai", "anthropic")
    pub name: String,
    /// Rate limits if configured
    pub rate_limits: Option<RateLimitsExport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub cached_input_per_1k_tokens: Option<f64>,
    pub currency: Currency,
    pub input_per_1k_tokens: f64,
    pub output_per_1k_tokens: f64,
}

/// Serializable rate limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitsExport {
    pub concurrent_requests: Option<i64>,
    pub requests_per_minute: Option<i64>,
    pub tokens_per_minute: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub context_window: i64,
    pub functions: bool,
    pub json_mode: bool,
    pub max_output_tokens: i64,
    pub multimodal: bool,
    pub streaming: bool,
    pub system_prompt: bool,
    pub vision: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraints {
    pub max_function_calls_per_message: Option<i64>,
    pub max_image_size_mb: Option<i64>,
    pub max_images_per_message: Option<i64>,
    pub supported_image_formats: Vec<String>,
}

/// Currency for pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "GBP")]
    GBP,
}

/// Summary statistics about the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStats {
    /// Total number of model families
    pub family_count: i64,
    /// Total number of models
    pub model_count: i64,
    /// Total number of services
    pub service_count: i64,
    /// Number of unverified models
    pub unverified_count: i64,
    /// Number of verified models
    pub verified_count: i64,
}

/// A position in the context tree - couples tree_id and node_id together.
/// This ensures we always have a valid reference into a specific tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// The specific node within the tree
    pub node_id: UUID,
    /// The tree containing this position
    pub tree_id: UUID,
}

/// Result of cone.set_head
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SetHeadResult {
    HeadUpdated {
cone_id: String,
new_head: Position,
old_head: Position,
    },
    Error {
message: String,
    },
}

/// Complete registry export - all services, families, and models in one structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryExport {
    /// All model families (e.g., "claude", "gpt", "deepseek")
    pub families: Vec<String>,
    /// All available models
    pub models: Vec<ModelExport>,
    /// All available services
    pub services: Vec<ServiceExport>,
    /// Summary statistics
    pub stats: RegistryStats,
}

/// Lightweight cone info (for listing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConeInfo {
    pub created_at: i64,
    pub head: Position,
    pub id: String,
    pub model_id: String,
    pub name: String,
}

// === Methods ===

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("cone.schema", serde_json::Value::Null).await
}

/// Delete a cone (associated tree is preserved)
pub async fn delete(client: &PlexusClient, identifier: ConeIdentifier) -> Result<DeleteResult> {
    client.call_single("cone.delete", json!({ "identifier": identifier })).await
}

/// Get cone configuration by name or ID
pub async fn get(client: &PlexusClient, identifier: ConeIdentifier) -> Result<GetResult> {
    client.call_single("cone.get", json!({ "identifier": identifier })).await
}

/// Create a new cone (LLM agent with persistent conversation context)
pub async fn create(client: &PlexusClient, metadata: serde_json::Value, model_id: String, name: String, system_prompt: Option<String>) -> Result<CreateResult> {
    client.call_single("cone.create", json!({ "metadata": metadata, "model_id": model_id, "name": name, "system_prompt": system_prompt })).await
}

/// Get available LLM services and models
pub async fn registry(client: &PlexusClient) -> Result<RegistryResult> {
    client.call_single("cone.registry", serde_json::Value::Null).await
}

/// Move cone's canonical head to a different node in the tree
pub async fn set_head(client: &PlexusClient, identifier: ConeIdentifier, node_id: UUID) -> Result<SetHeadResult> {
    client.call_single("cone.set_head", json!({ "identifier": identifier, "node_id": node_id })).await
}

/// List all cones
pub async fn list(client: &PlexusClient) -> Result<ListResult> {
    client.call_single("cone.list", serde_json::Value::Null).await
}

/// Chat with a cone - appends prompt to context, calls LLM, advances head
pub async fn chat(client: &PlexusClient, ephemeral: Option<bool>, identifier: ConeIdentifier, prompt: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChatEvent>> + Send>>> {
    let stream = client.call_stream("cone.chat", json!({ "ephemeral": ephemeral, "identifier": identifier, "prompt": prompt })).await?;

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
