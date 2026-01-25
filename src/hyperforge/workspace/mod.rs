//! Module for hyperforge.workspace namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// === Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ResolutionSource {

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceBinding {
    pub org_name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WorkspaceEvent {
    /// List of workspace bindings
    Listed {
bindings: Vec<WorkspaceBinding>,
    },
    /// Current workspace resolution
    Resolved {
org_name: String,
path: String,
source: ResolutionSource,
    },
    /// Workspace bound to org
    Bound {
org_name: String,
path: String,
    },
    /// Workspace unbound
    Unbound {
path: String,
    },
    /// No workspace binding found
    NotBound {
path: String,
    },
    /// Repos discovered during auto-create scan
    ReposDiscovered {
path: String,
repos: Vec<String>,
    },
    /// Repo staged during auto-create
    RepoStaged {
repo_name: String,
    },
    /// Error during workspace operation
    Error {
message: String,
    },
    /// Diff operation started for workspace
    DiffStarted {
org_count: i64,
workspace_path: String,
    },
    /// Diff result for a single org in the workspace
    OrgDiffResult {
in_sync: i64,
org_name: String,
to_create: i64,
to_delete: i64,
to_update: i64,
    },
    /// Org diff failed with error
    OrgDiffError {
message: String,
org_name: String,
    },
    /// Diff operation completed with summary
    DiffComplete {
total_in_sync: i64,
total_orgs: i64,
total_to_create: i64,
total_to_delete: i64,
total_to_update: i64,
    },
    /// Workspace import operation started
    ImportStarted {
org_count: i64,
workspace_path: String,
    },
    /// Started importing repos for a specific org
    OrgImportStarted {
org_name: String,
    },
    /// Completed importing repos for a specific org
    OrgImportComplete {
errors: i64,
imported: i64,
org_name: String,
skipped: i64,
    },
    /// Workspace import operation completed
    ImportComplete {
total_errors: i64,
total_imported: i64,
total_skipped: i64,
    },
    /// Clone all operation started
    CloneAllStarted {
org_count: i64,
    },
    /// Starting clone_all for a specific org
    OrgCloneAllStarted {
org_name: String,
    },
    /// Clone all complete for a specific org
    OrgCloneAllComplete {
cloned: i64,
failed: i64,
org_name: String,
skipped: i64,
    },
    /// Clone all operation complete for all orgs
    CloneAllComplete {
total_cloned: i64,
total_failed: i64,
total_skipped: i64,
    },
    /// Sync operation started for workspace
    SyncStarted {
org_count: i64,
workspace_path: String,
    },
    /// Sync started for a specific org within workspace sync
    OrgSyncStarted {
org_name: String,
    },
    /// Sync completed for a specific org within workspace sync
    OrgSyncComplete {
failed: i64,
org_name: String,
synced: i64,
unchanged: i64,
    },
    /// Sync operation completed for entire workspace
    SyncComplete {
total_failed: i64,
total_synced: i64,
total_unchanged: i64,
workspace_path: String,
    },
}

// === Methods ===

/// Clone all repos for all orgs bound to current workspace
pub async fn clone_all(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.clone_all", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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

/// Show diff for all orgs bound to current workspace
pub async fn diff(client: &PlexusClient, path: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.diff", json!({ "path": path })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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

/// Show current workspace resolution
pub async fn show(client: &PlexusClient, path: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.show", json!({ "path": path })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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

/// Remove a workspace binding
pub async fn unbind(client: &PlexusClient, path: String) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.unbind", json!({ "path": path })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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

/// Import repos for all orgs bound to current workspace
pub async fn import(client: &PlexusClient, include_private: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.import", json!({ "include_private": include_private })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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

/// List all workspace bindings
pub async fn list(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.list", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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

/// Bind a directory to an organization
pub async fn bind(client: &PlexusClient, auto_create: Option<bool>, org_name: String, path: String) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.bind", json!({ "auto_create": auto_create, "org_name": org_name, "path": path })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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
    client.call_single("hyperforge.workspace.schema", serde_json::Value::Null).await
}

/// Sync repos for all orgs bound to current workspace
pub async fn sync(client: &PlexusClient, yes: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<WorkspaceEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.workspace.sync", json!({ "yes": yes })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<WorkspaceEvent>(content) {
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
