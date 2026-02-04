//! Module for health namespace
//! Do not edit manually

use crate::client::PlexusClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// === Types ===

/// Stream events from health check
/// 
/// This is a plain domain type - no trait implementations needed.
/// The caller (Plexus) wraps this with metadata when streaming.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HealthEvent {
    /// Current health status
    Status {
status: String,
timestamp: i64,
uptime_seconds: u64,
    },
}

// === Methods ===

/// Check the health status of the Plexus RPC server and return uptime
pub async fn check(client: &PlexusClient) -> Result<HealthEvent> {
    client.call_single("health.check", serde_json::Value::Null).await
}

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("health.schema", serde_json::Value::Null).await
}
