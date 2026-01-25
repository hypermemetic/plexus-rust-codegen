//! Module for hyperforge.forge namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// Child namespaces
pub mod codeberg;
pub mod github;

use crate::hyperforge::org::Forge;

// === Types ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ForgeEvent {
    /// List of repositories on forge
    ReposListed {
forge: Forge,
owner: String,
repos: Vec<ForgeRepoSummary>,
    },
    /// Repository created on forge
    RepoCreated {
forge: Forge,
owner: String,
repo_name: String,
url: String,
    },
    /// Authentication status
    AuthStatus {
authenticated: bool,
forge: Forge,
scopes: Vec<String>,
user: Option<String>,
    },
    /// API call progress
    ApiProgress {
forge: Forge,
message: String,
operation: String,
    },
    /// API error
    Error {
forge: Forge,
message: String,
operation: String,
status_code: Option<i64>,
    },
    /// Auth check started
    AuthStarted {
forge: Forge,
org_name: String,
    },
    /// Auth check completed with result
    AuthResult {
forge: Forge,
last_validated: Option<String>,
org_name: String,
scopes: Vec<String>,
status: TokenStatus,
username: Option<String>,
    },
    /// Auth check failed (network/internal error, not auth failure)
    AuthFailed {
error: String,
forge: Forge,
org_name: String,
    },
    /// Token refresh started
    RefreshStarted {
forge: Forge,
org_name: String,
    },
    /// Token refresh completed successfully
    RefreshComplete {
forge: Forge,
org_name: String,
status: TokenStatus,
    },
    /// Token refresh failed
    RefreshFailed {
error: String,
forge: Forge,
org_name: String,
    },
}

/// Summary of a repository on a forge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeRepoSummary {
    pub description: Option<String>,
    pub name: String,
    pub private: bool,
    pub url: String,
}

/// Status of a token for a specific org/forge combination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TokenStatus {

}

// === Methods ===

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("hyperforge.forge.schema", serde_json::Value::Null).await
}

/// Check authentication status for a forge
pub async fn auth(client: &PlexusClient, forge: String, org: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<ForgeEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.forge.auth", json!({ "forge": forge, "org": org })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ForgeEvent>(content) {
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

/// Refresh/update token for a forge
pub async fn refresh(client: &PlexusClient, forge: String, org: Option<String>, token: Option<String>) -> Result<Pin<Box<dyn Stream<Item = Result<ForgeEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.forge.refresh", json!({ "forge": forge, "org": org, "token": token })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ForgeEvent>(content) {
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

/// List supported forges
pub async fn list(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<ForgeEvent>> + Send>>> {
    let stream = client.call_stream("hyperforge.forge.list", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ForgeEvent>(content) {
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
