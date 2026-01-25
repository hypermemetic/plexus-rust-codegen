//! Module for hyperforge.org.juggernautlabs.repos namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

use crate::hyperforge::org::hypermemetic::repos::{RepoAdoptEvent, RepoCloneEvent, RepoConvergeEvent, RepoCreateEvent, RepoDiffEvent, RepoListEvent, RepoRefreshEvent, RepoRemoveEvent, RepoSyncEvent};

// === Methods ===

/// Create/update a repository configuration
pub async fn create(client: &PlexusClient, description: Option<String>, forges: Option<String>, init_local: Option<bool>, path: Option<String>, repo_name: String, visibility: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoCreateEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.create", json!({ "description": description, "forges": forges, "init_local": init_local, "path": path, "repo_name": repo_name, "visibility": visibility })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoCreateEvent>(content) {
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

/// Full bidirectional sync with convergence verification
pub async fn converge(client: &PlexusClient, dry_run: Option<bool>, yes: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoConvergeEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.converge", json!({ "dry_run": dry_run, "yes": yes })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoConvergeEvent>(content) {
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

/// Adopt an existing directory as a hyperforge-managed repository
pub async fn adopt(client: &PlexusClient, description: Option<String>, forges: Option<String>, git_init: Option<bool>, path: String, repo_name: String, scan_packages: Option<bool>, visibility: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoAdoptEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.adopt", json!({ "description": description, "forges": forges, "git_init": git_init, "path": path, "repo_name": repo_name, "scan_packages": scan_packages, "visibility": visibility })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoAdoptEvent>(content) {
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
    client.call_single("hyperforge.org.juggernautlabs.repos.schema", serde_json::Value::Null).await
}

/// Clone all repositories for an organization
pub async fn clone_all(client: &PlexusClient, target: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoCloneEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.clone_all", json!({ "target": target })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoCloneEvent>(content) {
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

/// Sync repositories to forges
pub async fn sync(client: &PlexusClient, dry_run: Option<bool>, repo_name: Option<String>, yes: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoSyncEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.sync", json!({ "dry_run": dry_run, "repo_name": repo_name, "yes": yes })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoSyncEvent>(content) {
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

/// Mark a repository for deletion
pub async fn remove(client: &PlexusClient, force: Option<bool>, repo_name: String) -> Result<Pin<Box<dyn Stream<Item = Result<RepoRemoveEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.remove", json!({ "force": force, "repo_name": repo_name })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoRemoveEvent>(content) {
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

/// Compare local desired state vs remote actual state
pub async fn diff(client: &PlexusClient, refresh: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoDiffEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.diff", json!({ "refresh": refresh })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoDiffEvent>(content) {
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

/// List repositories in this organization
pub async fn list(client: &PlexusClient, staged: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoListEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.list", json!({ "staged": staged })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoListEvent>(content) {
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

/// Refresh local state from forge APIs
pub async fn refresh(client: &PlexusClient, force: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoRefreshEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.refresh", json!({ "_force": force })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoRefreshEvent>(content) {
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

/// Clone a repository from forges and configure remotes
pub async fn clone(client: &PlexusClient, repo_name: String, target: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoCloneEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.juggernautlabs.repos.clone", json!({ "repo_name": repo_name, "target": target })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<RepoCloneEvent>(content) {
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
