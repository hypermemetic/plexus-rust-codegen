//! Module for hyperforge.org namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// Child namespaces
pub mod hypermemetic;
pub mod juggernautlabs;

// === Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum OrgEvent {
    /// List of all organizations
    Listed {
orgs: Vec<OrgSummary>,
    },
    /// Details of a single organization
    Details {
org: Org,
    },
    /// Organization created
    Created {
org_name: String,
    },
    /// Organization removed
    Removed {
org_name: String,
    },
    /// Organization updated
    Updated {
field: String,
org_name: String,
value: String,
    },
    /// Informational message about organization
    Info {
message: String,
name: String,
    },
    /// Import started
    ImportStarted {
forges: Vec<Forge>,
org_name: String,
    },
    /// Repository discovered during import
    RepoImported {
description: Option<String>,
forges: Vec<Forge>,
org_name: String,
repo_name: String,
visibility: Visibility,
    },
    /// Import completed
    ImportComplete {
imported_count: i64,
org_name: String,
skipped_count: i64,
    },
    /// Error occurred
    Error {
message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Org {
    pub default_visibility: Visibility,
    /// Forges configuration - supports both legacy array format and new object format
    pub forges: ForgesConfig,
    pub name: String,
    pub origin: Forge,
    pub owner: String,
    pub ssh_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

/// Forges configuration that supports both legacy array format and new object format.
/// 
/// Legacy format (array of forge names):
/// ```yaml
/// forges:
/// - github
/// - codeberg
/// ```
/// 
/// New format (object with per-forge config):
/// ```yaml
/// forges:
/// github:
/// sync: true
/// codeberg:
/// sync: false
/// ```
pub type ForgesConfig = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgSummary {
    pub forges: ForgesConfig,
    pub name: String,
    pub owner: String,
}

/// Per-forge configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    /// Whether to sync repositories to this forge (default: true)
    pub sync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Forge {
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "codeberg")]
    Codeberg,
    #[serde(rename = "gitlab")]
    Gitlab,
}

// === Methods ===

/// Show details of a specific organization
pub async fn show(client: &PlexusClient, org_name: String) -> Result<Pin<Box<dyn Stream<Item = Result<OrgEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.show", json!({ "org_name": org_name })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<OrgEvent>(content) {
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

/// Remove an organization
pub async fn remove(client: &PlexusClient, org_name: String) -> Result<Pin<Box<dyn Stream<Item = Result<OrgEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.remove", json!({ "org_name": org_name })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<OrgEvent>(content) {
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

/// List all configured organizations
pub async fn list(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<OrgEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.list", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<OrgEvent>(content) {
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

/// Import repositories from existing forges
pub async fn import(client: &PlexusClient, dry_run: Option<bool>, include_private: Option<bool>, org_name: String) -> Result<Pin<Box<dyn Stream<Item = Result<OrgEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.import", json!({ "dry_run": dry_run, "include_private": include_private, "org_name": org_name })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<OrgEvent>(content) {
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
    client.call_single("hyperforge.org.schema", serde_json::Value::Null).await
}

/// Create a new organization
pub async fn create(client: &PlexusClient, default_visibility: Option<String>, forges: String, org_name: String, origin: String, owner: String, ssh_key: String) -> Result<Pin<Box<dyn Stream<Item = Result<OrgEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.create", json!({ "default_visibility": default_visibility, "forges": forges, "org_name": org_name, "origin": origin, "owner": owner, "ssh_key": ssh_key })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<OrgEvent>(content) {
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
