//! Module for arbor namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

use crate::cone::UUID;

// === Types ===

/// Events emitted by Arbor operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ArborEvent {
    TreeCreated {
tree_id: UUID,
    },
    TreeDeleted {
tree_id: UUID,
    },
    TreeUpdated {
tree_id: UUID,
    },
    TreeList {
tree_ids: Vec<UUID>,
    },
    TreeClaimed {
new_count: i64,
owner_id: String,
tree_id: UUID,
    },
    TreeReleased {
new_count: i64,
owner_id: String,
tree_id: UUID,
    },
    TreeScheduledDeletion {
scheduled_at: i64,
tree_id: UUID,
    },
    TreeArchived {
archived_at: i64,
tree_id: UUID,
    },
    TreeRefs {
refs: ResourceRefs,
tree_id: UUID,
    },
    NodeCreated {
node_id: UUID,
parent: Option<UUID>,
tree_id: UUID,
    },
    NodeUpdated {
new_id: UUID,
old_id: UUID,
tree_id: UUID,
    },
    NodeDeleted {
node_id: UUID,
tree_id: UUID,
    },
    NodeClaimed {
new_count: i64,
node_id: UUID,
owner_id: String,
tree_id: UUID,
    },
    NodeReleased {
new_count: i64,
node_id: UUID,
owner_id: String,
tree_id: UUID,
    },
    NodeScheduledDeletion {
node_id: UUID,
scheduled_at: i64,
tree_id: UUID,
    },
    NodeArchived {
archived_at: i64,
node_id: UUID,
tree_id: UUID,
    },
    NodeRefs {
node_id: UUID,
refs: ResourceRefs,
tree_id: UUID,
    },
    TreeData {
tree: Tree,
    },
    TreeSkeleton {
skeleton: TreeSkeleton,
    },
    NodeData {
node: Node,
tree_id: UUID,
    },
    NodeChildren {
children: Vec<UUID>,
node_id: UUID,
tree_id: UUID,
    },
    NodeParent {
node_id: UUID,
parent: Option<UUID>,
tree_id: UUID,
    },
    ContextPath {
path: Vec<UUID>,
tree_id: UUID,
    },
    ContextPathData {
nodes: Vec<Node>,
tree_id: UUID,
    },
    ContextHandles {
handles: Vec<Handle>,
tree_id: UUID,
    },
    ContextLeaves {
leaves: Vec<UUID>,
tree_id: UUID,
    },
    TreesScheduled {
tree_ids: Vec<UUID>,
    },
    NodesScheduled {
node_ids: Vec<UUID>,
tree_id: UUID,
    },
    TreesArchived {
tree_ids: Vec<UUID>,
    },
    TreeRender {
render: String,
tree_id: UUID,
    },
}

/// Reference counting information for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRefs {
    /// Who owns references (owner_id -> count)
    pub owners: serde_json::Value,
    /// Total reference count
    pub ref_count: i64,
}

/// Resource state in deletion lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "scheduled_delete")]
    ScheduledDelete,
    #[serde(rename = "archived")]
    Archived,
}

/// Lightweight node representation (just structure, no data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSkeleton {
    pub children: Vec<UUID>,
    pub id: UUID,
    /// Type indicator (but not the actual data)
    pub node_type: String,
    pub parent: Option<UUID>,
}

/// Node type discriminator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NodeType {
    /// Built-in text node (data stored in Arbor)
    Text {
content: String,
    },
    /// External data reference
    External {
handle: Handle,
    },
}

/// A node in the conversation tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Archived timestamp (Unix seconds)
    pub archived_at: Option<i64>,
    /// Child nodes (in order)
    pub children: Vec<UUID>,
    /// Creation timestamp (Unix seconds)
    pub created_at: i64,
    /// Node data (handle or built-in)
    pub data: NodeType,
    /// Unique identifier for this node
    pub id: UUID,
    /// Optional metadata
    pub metadata: serde_json::Value,
    /// Parent node (None for root)
    pub parent: Option<UUID>,
    /// Reference count information
    pub refs: Option<ResourceRefs>,
    /// Scheduled deletion timestamp (Unix seconds)
    pub scheduled_deletion_at: Option<i64>,
    /// Reference counting state
    pub state: Option<ResourceState>,
}

/// A conversation tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    /// Archived timestamp (Unix seconds)
    pub archived_at: Option<i64>,
    /// Creation timestamp (Unix seconds)
    pub created_at: i64,
    /// Unique identifier for this tree
    pub id: UUID,
    /// Optional tree-level metadata (name, description, etc.)
    pub metadata: serde_json::Value,
    /// All nodes in the tree (NodeId -> Node)
    pub nodes: serde_json::Value,
    /// Reference count information
    pub refs: Option<ResourceRefs>,
    /// Root node ID
    pub root: UUID,
    /// Scheduled deletion timestamp (Unix seconds)
    pub scheduled_deletion_at: Option<i64>,
    /// Reference counting state
    pub state: Option<ResourceState>,
    /// Last modified timestamp (Unix seconds)
    pub updated_at: i64,
}

/// Lightweight tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeSkeleton {
    pub id: UUID,
    pub nodes: serde_json::Value,
    pub root: UUID,
    pub state: Option<ResourceState>,
}

/// Handle pointing to external data with versioning
/// 
/// Display format: `{plugin_id}@{version}::{method}:meta[0]:meta[1]:...`
/// 
/// Examples:
/// - `550e8400-e29b-41d4-a716-446655440000@1.0.0::chat:msg-123:user:bob`
/// - `123e4567-e89b-12d3-a456-426614174000@1.0.0::execute:cmd-789`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    /// Metadata parts - variable length list of strings
    /// For messages: typically [message_uuid, role, optional_extra...]
    pub meta: Vec<String>,
    /// Creation method that produced this handle (e.g., "chat", "execute")
    pub method: String,
    /// Stable plugin instance identifier (UUID)
    pub plugin_id: String,
    /// Plugin version (semantic version: "MAJOR.MINOR.PATCH")
    /// Used for schema/type lookup
    pub version: String,
}

// === Methods ===

/// Get the path from root to a node
pub async fn node_get_path(client: &PlexusClient, node_id: UUID, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.node_get_path", json!({ "node_id": node_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// List all active trees
pub async fn tree_list(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_list", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// List all leaf nodes in a tree
pub async fn context_list_leaves(client: &PlexusClient, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.context_list_leaves", json!({ "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Get lightweight tree structure without node data
pub async fn tree_get_skeleton(client: &PlexusClient, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_get_skeleton", json!({ "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Retrieve a complete tree with all nodes
pub async fn tree_get(client: &PlexusClient, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_get", json!({ "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Create a text node in a tree
pub async fn node_create_text(client: &PlexusClient, content: String, metadata: serde_json::Value, parent: Option<UUID>, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.node_create_text", json!({ "content": content, "metadata": metadata, "parent": parent, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Get the children of a node
pub async fn node_get_children(client: &PlexusClient, node_id: UUID, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.node_get_children", json!({ "node_id": node_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Release ownership of a tree (decrement reference count)
pub async fn tree_release(client: &PlexusClient, count: i64, owner_id: String, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_release", json!({ "count": count, "owner_id": owner_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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
    client.call_single("arbor.schema", serde_json::Value::Null).await
}

/// Get the parent of a node
pub async fn node_get_parent(client: &PlexusClient, node_id: UUID, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.node_get_parent", json!({ "node_id": node_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Render tree as text visualization  If parent context is available, automatically resolves handles to show actual content. Otherwise, shows handle references.
pub async fn tree_render(client: &PlexusClient, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_render", json!({ "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Create an external node in a tree
pub async fn node_create_external(client: &PlexusClient, handle: Handle, metadata: serde_json::Value, parent: Option<UUID>, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.node_create_external", json!({ "handle": handle, "metadata": metadata, "parent": parent, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Get all external handles in the path to a node
pub async fn context_get_handles(client: &PlexusClient, node_id: UUID, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.context_get_handles", json!({ "node_id": node_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Get a node by ID
pub async fn node_get(client: &PlexusClient, node_id: UUID, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.node_get", json!({ "node_id": node_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Get the full path data from root to a node
pub async fn context_get_path(client: &PlexusClient, node_id: UUID, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.context_get_path", json!({ "node_id": node_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// List trees scheduled for deletion
pub async fn tree_list_scheduled(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_list_scheduled", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Claim ownership of a tree (increment reference count)
pub async fn tree_claim(client: &PlexusClient, count: i64, owner_id: String, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_claim", json!({ "count": count, "owner_id": owner_id, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Update tree metadata
pub async fn tree_update_metadata(client: &PlexusClient, metadata: serde_json::Value, tree_id: UUID) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_update_metadata", json!({ "metadata": metadata, "tree_id": tree_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// Create a new conversation tree
pub async fn tree_create(client: &PlexusClient, metadata: serde_json::Value, owner_id: String) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_create", json!({ "metadata": metadata, "owner_id": owner_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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

/// List archived trees
pub async fn tree_list_archived(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<ArborEvent>> + Send>>> {
    let stream = client.call_stream("arbor.tree_list_archived", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<ArborEvent>(content) {
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
