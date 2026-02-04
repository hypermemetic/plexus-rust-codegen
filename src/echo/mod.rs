//! Module for echo namespace
//! Do not edit manually

use crate::client::PlexusClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

// === Types ===

/// Events from echo operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum EchoEvent {
    /// Echo response
    Echo {
        /// Number of times repeated
count: i64,
        /// The echoed message
message: String,
    },
}

// === Methods ===

/// Echo a simple message once
pub async fn once(client: &PlexusClient, message: String) -> Result<EchoEvent> {
    client.call_single("echo.once", json!({ "message": message })).await
}

/// Echo a message back
pub async fn echo(client: &PlexusClient, count: i64, message: String) -> Result<EchoEvent> {
    client.call_single("echo.echo", json!({ "count": count, "message": message })).await
}

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("echo.schema", serde_json::Value::Null).await
}
