//! Module for hyperforge.org.hypermemetic.repos namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

use crate::hyperforge::org::{Forge, Visibility};

// === Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoAdoptEvent {
    /// Adopt operation started
    Started {
org_name: String,
path: String,
repo_name: String,
    },
    /// Git repository initialized during adopt
    GitInitialized {
org_name: String,
path: String,
repo_name: String,
    },
    /// Git remotes detected during adopt
    RemotesDetected {
forges: Vec<Forge>,
org_name: String,
repo_name: String,
    },
    /// Packages detected during adopt
    PackagesDetected {
org_name: String,
packages: Vec<PackageConfig>,
repo_name: String,
    },
    /// Adopt operation completed
    Complete {
org_name: String,
path: String,
repo_name: String,
    },
    /// Error during adopt operation
    Error {
message: String,
org_name: String,
repo_name: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoDiffEvent {
    /// Diff result for a single repository
    RepoDiff {
details: Vec<String>,
org_name: String,
repo_name: String,
status: DiffStatus,
    },
    /// Overall diff summary for an organization
    Summary {
in_sync: i64,
org_name: String,
to_create: i64,
to_delete: i64,
to_update: i64,
untracked: i64,
    },
    /// Error during diff operation
    Error {
message: String,
org_name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoCreateEvent {
    /// Repository staged for creation
    Staged {
org_name: String,
repo_name: String,
    },
    /// Local git repository initialized
    LocalInitialized {
org_name: String,
path: String,
repo_name: String,
    },
    /// .gitignore file created
    GitignoreCreated {
org_name: String,
path: String,
repo_name: String,
    },
    /// Git remote added to local repository
    RemoteAdded {
org_name: String,
remote: String,
repo_name: String,
url: String,
    },
    /// Local repository setup complete
    LocalSetupComplete {
org_name: String,
path: String,
repo_name: String,
    },
    /// Error during create operation
    Error {
message: String,
org_name: String,
repo_name: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSummary {
    pub forges: Vec<Forge>,
    pub name: String,
    pub synced: bool,
    pub visibility: Visibility,
}

/// Type of package/registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageType {
    #[serde(rename = "crate")]
    Crate,
    #[serde(rename = "npm")]
    Npm,
    #[serde(rename = "hex")]
    Hex,
    #[serde(rename = "hackage")]
    Hackage,
    #[serde(rename = "pypi")]
    Pypi,
}

/// Result of a convergence operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergeResult {
    /// Whether the system converged (no remaining drift)
    pub converged: bool,
    /// Whether drift was detected after apply
    pub drift_detected: bool,
    /// Number of repos created on forges
    pub repos_created: i64,
    /// Number of repos deleted from forges
    pub repos_deleted: i64,
    /// Number of repos synced (created + updated)
    pub repos_synced: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoDetails {
    pub description: Option<String>,
    pub forge_urls: serde_json::Value,
    pub name: String,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoSyncEvent {
    /// Sync operation started
    Started {
org_name: String,
repo_count: i64,
    },
    /// Sync progress update
    Progress {
org_name: String,
repo_name: String,
stage: String,
    },
    /// Repository synced to forge
    Synced {
forge: Forge,
org_name: String,
repo_name: String,
url: String,
    },
    /// Forge skipped due to sync flag being false
    ForgeSkipped {
forge: Forge,
org_name: String,
reason: String,
    },
    /// Outputs captured from Pulumi after apply
    OutputsCaptured {
forge: Forge,
id: Option<String>,
org_name: String,
repo_name: String,
url: String,
    },
    /// Sync operation completed
    Complete {
org_name: String,
success: bool,
synced_count: i64,
    },
    /// Error during sync operation
    Error {
message: String,
org_name: String,
repo_name: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoConvergeEvent {
    /// Converge operation started
    Started {
org_name: String,
phases: Vec<String>,
    },
    /// Converge phase status update
    Phase {
org_name: String,
phase: String,
status: String,
    },
    /// Converge operation completed
    Complete {
changes_applied: i64,
final_state: ConvergeResult,
org_name: String,
success: bool,
    },
    /// Error during converge operation
    Error {
message: String,
org_name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoRemoveEvent {
    /// Repository marked for deletion
    MarkedForDeletion {
org_name: String,
repo_name: String,
    },
    /// Repository protection error (protected repos require --force)
    ProtectionError {
message: String,
org_name: String,
repo_name: String,
    },
    /// Repository removed
    Removed {
org_name: String,
repo_name: String,
    },
    /// Error during remove operation
    Error {
message: String,
org_name: String,
repo_name: Option<String>,
    },
}

/// Status of a repository in a diff operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffStatus {
    #[serde(rename = "to_create")]
    ToCreate,
    #[serde(rename = "to_update")]
    ToUpdate,
    #[serde(rename = "to_delete")]
    ToDelete,
    #[serde(rename = "in_sync")]
    InSync,
    #[serde(rename = "untracked")]
    Untracked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoCloneEvent {
    /// Clone operation started
    Started {
org_name: String,
repo_name: String,
target_path: String,
    },
    /// Clone progress - cloning from forge
    Progress {
forge: Forge,
org_name: String,
repo_name: String,
stage: String,
    },
    /// Remote added during clone
    RemoteAdded {
org_name: String,
remote_name: String,
repo_name: String,
url: String,
    },
    /// Git remotes validated for a repository
    RemotesValidated {
org_name: String,
remotes: Vec<String>,
repo_name: String,
    },
    /// Remote sync status (comparing refs across remotes)
    RemoteSyncStatus {
branch: String,
details: Vec<String>,
in_sync: bool,
org_name: String,
repo_name: String,
    },
    /// Clone completed successfully
    Complete {
org_name: String,
remotes: Vec<String>,
repo_name: String,
target_path: String,
    },
    /// Error during clone operation
    Error {
message: String,
org_name: String,
repo_name: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoRefreshEvent {
    /// Refresh started - querying forges for remote state
    Started {
forges: Vec<Forge>,
org_name: String,
    },
    /// Forge query progress during refresh
    Progress {
forge: Forge,
org_name: String,
repos_found: i64,
    },
    /// Refresh completed with discovery statistics
    Complete {
discovered: i64,
matched: i64,
org_name: String,
untracked: i64,
    },
    /// Error during refresh operation
    Error {
message: String,
org_name: String,
    },
}

/// Configuration for a package within a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    /// Package name (as published to registry)
    pub name: String,
    /// Path to package root relative to repo root (default: ".")
    pub path: String,
    /// Whether to publish this package (default: true)
    pub publish: bool,
    /// Custom publish command (overrides default)
    pub publish_command: Option<String>,
    /// Registry to publish to (default: type's default registry)
    pub registry: Option<String>,
    /// Type of package
    pub r#type: PackageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RepoListEvent {
    /// List of repositories
    Listed {
org_name: String,
repos: Vec<RepoSummary>,
staged: bool,
    },
    /// Details of a single repository
    Details {
org_name: String,
repo: RepoDetails,
    },
    /// Error during list operation
    Error {
message: String,
org_name: String,
    },
}

// === Methods ===

/// Full bidirectional sync with convergence verification
pub async fn converge(client: &PlexusClient, dry_run: Option<bool>, yes: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoConvergeEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.converge", json!({ "dry_run": dry_run, "yes": yes })).await?;

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

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("hyperforge.org.hypermemetic.repos.schema", serde_json::Value::Null).await
}

/// Clone all repositories for an organization
pub async fn clone_all(client: &PlexusClient, target: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoCloneEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.clone_all", json!({ "target": target })).await?;

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

/// List repositories in this organization
pub async fn list(client: &PlexusClient, staged: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoListEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.list", json!({ "staged": staged })).await?;

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

/// Adopt an existing directory as a hyperforge-managed repository
pub async fn adopt(client: &PlexusClient, description: Option<String>, forges: Option<String>, git_init: Option<bool>, path: String, repo_name: String, scan_packages: Option<bool>, visibility: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoAdoptEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.adopt", json!({ "description": description, "forges": forges, "git_init": git_init, "path": path, "repo_name": repo_name, "scan_packages": scan_packages, "visibility": visibility })).await?;

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

/// Compare local desired state vs remote actual state
pub async fn diff(client: &PlexusClient, refresh: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoDiffEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.diff", json!({ "refresh": refresh })).await?;

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

/// Mark a repository for deletion
pub async fn remove(client: &PlexusClient, force: Option<bool>, repo_name: String) -> Result<Pin<Box<dyn Stream<Item = Result<RepoRemoveEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.remove", json!({ "force": force, "repo_name": repo_name })).await?;

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

/// Clone a repository from forges and configure remotes
pub async fn clone(client: &PlexusClient, repo_name: String, target: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoCloneEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.clone", json!({ "repo_name": repo_name, "target": target })).await?;

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

/// Create/update a repository configuration
pub async fn create(client: &PlexusClient, description: Option<String>, forges: Option<String>, init_local: Option<bool>, path: Option<String>, repo_name: String, visibility: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoCreateEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.create", json!({ "description": description, "forges": forges, "init_local": init_local, "path": path, "repo_name": repo_name, "visibility": visibility })).await?;

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

/// Refresh local state from forge APIs
pub async fn refresh(client: &PlexusClient, force: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoRefreshEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.refresh", json!({ "_force": force })).await?;

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

/// Sync repositories to forges
pub async fn sync(client: &PlexusClient, dry_run: Option<bool>, repo_name: Option<String>, yes: Option<bool>) -> Result<Pin<Box<dyn Stream<Item = Result<RepoSyncEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.org.hypermemetic.repos.sync", json!({ "dry_run": dry_run, "repo_name": repo_name, "yes": yes })).await?;

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
