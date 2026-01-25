//! Module for changelog namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// === Types ===

/// A changelog entry documenting a plexus hash transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    /// Who/what added this entry
    pub author: Option<String>,
    /// Unix timestamp when this entry was added
    pub created_at: i64,
    /// Detailed list of changes (bullet points)
    pub details: Vec<String>,
    /// The plexus_hash this entry documents
    pub hash: String,
    /// The previous hash this transitioned from (None for initial entry)
    pub previous_hash: Option<String>,
    /// Reference to a queue item this changelog completes
    pub queue_id: Option<String>,
    /// Short summary of changes (one line)
    pub summary: String,
}

/// Events emitted by changelog operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChangelogEvent {
    /// Entry was added
    EntryAdded {
entry: ChangelogEntry,
    },
    /// List of entries
    Entries {
entries: Vec<ChangelogEntry>,
    },
    /// Current state check result
    Status {
current_hash: String,
entry: Option<ChangelogEntry>,
is_documented: bool,
previous_hash: Option<String>,
    },
    /// Startup check result
    StartupCheck {
current_hash: String,
hash_changed: bool,
is_documented: bool,
message: String,
previous_hash: Option<String>,
    },
    /// Queue item was added
    QueueAdded {
entry: QueueEntry,
    },
    /// Queue item was updated (e.g., marked complete)
    QueueUpdated {
entry: QueueEntry,
    },
    /// List of queue items
    QueueEntries {
entries: Vec<QueueEntry>,
    },
    /// Single queue item
    QueueItem {
entry: Option<QueueEntry>,
    },
}

/// A queued change - a planned modification that systems should implement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueEntry {
    /// Unix timestamp when this was completed
    pub completed_at: Option<i64>,
    /// The hash where this change was implemented (set when completed)
    pub completed_hash: Option<String>,
    /// Unix timestamp when this was queued
    pub created_at: i64,
    /// Description of the planned change
    pub description: String,
    /// Unique identifier for this queue item
    pub id: String,
    /// Current status of the queue item
    pub status: QueueStatus,
    /// Tags to identify which systems this change affects (e.g., "frontend", "api", "breaking")
    pub tags: Vec<String>,
}

/// Status of a queued change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
}

// === Methods ===

/// Get a specific changelog entry by hash
pub async fn get(client: &PlexusClient, hash: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.get", json!({ "hash": hash })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// List all queue entries, optionally filtered by tag
pub async fn queue_list(client: &PlexusClient, tag: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.queue_list", json!({ "tag": tag })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// List all changelog entries
pub async fn list(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.list", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// Check current status - is the current plexus hash documented?
pub async fn check(client: &PlexusClient, current_hash: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.check", json!({ "current_hash": current_hash })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// List pending queue entries, optionally filtered by tag
pub async fn queue_pending(client: &PlexusClient, tag: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.queue_pending", json!({ "tag": tag })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// Mark a queue entry as complete
pub async fn queue_complete(client: &PlexusClient, hash: String, id: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.queue_complete", json!({ "hash": hash, "id": id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// Add a planned change to the queue
pub async fn queue_add(client: &PlexusClient, description: String, tags: Option<Vec<serde_json::Value>>) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.queue_add", json!({ "description": description, "tags": tags })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// Add a changelog entry for a plexus hash transition
pub async fn add(client: &PlexusClient, author: Option<String>, details: Option<Vec<serde_json::Value>>, hash: String, previous_hash: Option<String>, queue_id: Option<String>, summary: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.add", json!({ "author": author, "details": details, "hash": hash, "previous_hash": previous_hash, "queue_id": queue_id, "summary": summary })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("changelog.schema", serde_json::Value::Null).await
}

/// Get a specific queue entry by ID
pub async fn queue_get(client: &PlexusClient, id: String) -> Result<Pin<Box<dyn Stream<Item = Result<ChangelogEvent>> + Send>>> {
    let stream = client.call_stream("changelog.queue_get", json!({ "id": id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ChangelogEvent>(content) {
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
