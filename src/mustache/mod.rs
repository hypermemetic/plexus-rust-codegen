//! Module for mustache namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// === Types ===

/// Information about a registered template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// When the template was created (Unix timestamp)
    pub created_at: i64,
    /// Unique template ID
    pub id: String,
    /// Method this template is for
    pub method: String,
    /// Template name (e.g., "default", "compact", "verbose")
    pub name: String,
    /// Plugin that owns this template
    pub plugin_id: String,
    /// When the template was last updated (Unix timestamp)
    pub updated_at: i64,
}

/// Events from mustache operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MustacheEvent {
    /// Template rendered successfully
    Rendered {
        /// The rendered output
output: String,
    },
    /// Template registered successfully
    Registered {
        /// Template info
template: TemplateInfo,
    },
    /// Template retrieved
    Template {
        /// The template content
template: String,
    },
    /// Template not found
    NotFound {
        /// Description of what was not found
message: String,
    },
    /// List of templates
    Templates {
        /// The templates
templates: Vec<TemplateInfo>,
    },
    /// Template deleted
    Deleted {
        /// Number of templates deleted
count: i64,
    },
    /// Error occurred
    Error {
        /// Error message
message: String,
    },
}

// === Methods ===

/// Get plugin or method schema. Pass {"method": "name"} for a specific method.
pub async fn schema(client: &PlexusClient) -> Result<serde_json::Value> {
    client.call_single("mustache.schema", serde_json::Value::Null).await
}

/// Delete a template
pub async fn delete_template(client: &PlexusClient, method: String, name: String, plugin_id: String) -> Result<Pin<Box<dyn Stream<Item = Result<MustacheEvent>> + Send>>> {
    let stream = client.call_stream("mustache.delete_template", json!({ "method": method, "name": name, "plugin_id": plugin_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<MustacheEvent>(content) {
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

/// Render a value using a template  Looks up the template for the given plugin/method/name combination and renders the value using mustache templating. If template_name is None, uses "default".
pub async fn render(client: &PlexusClient, method: String, plugin_id: String, template_name: Option<String>, value: serde_json::Value) -> Result<Pin<Box<dyn Stream<Item = Result<MustacheEvent>> + Send>>> {
    let stream = client.call_stream("mustache.render", json!({ "method": method, "plugin_id": plugin_id, "template_name": template_name, "value": value })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<MustacheEvent>(content) {
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

/// List all templates for a plugin
pub async fn list_templates(client: &PlexusClient, plugin_id: String) -> Result<Pin<Box<dyn Stream<Item = Result<MustacheEvent>> + Send>>> {
    let stream = client.call_stream("mustache.list_templates", json!({ "plugin_id": plugin_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<MustacheEvent>(content) {
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

/// Register a template for a plugin/method  Templates are identified by (plugin_id, method, name). If a template with the same identifier already exists, it will be updated.
pub async fn register_template(client: &PlexusClient, method: String, name: String, plugin_id: String, template: String) -> Result<Pin<Box<dyn Stream<Item = Result<MustacheEvent>> + Send>>> {
    let stream = client.call_stream("mustache.register_template", json!({ "method": method, "name": name, "plugin_id": plugin_id, "template": template })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<MustacheEvent>(content) {
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

/// Get a specific template
pub async fn get_template(client: &PlexusClient, method: String, name: String, plugin_id: String) -> Result<Pin<Box<dyn Stream<Item = Result<MustacheEvent>> + Send>>> {
    let stream = client.call_stream("mustache.get_template", json!({ "method": method, "name": name, "plugin_id": plugin_id })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<MustacheEvent>(content) {
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
