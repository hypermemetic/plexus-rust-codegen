//! Module for solar namespace
//! Do not edit manually

use crate::client::PlexusClient;
use crate::types::*;
use anyhow::{anyhow, Result};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::pin::Pin;

// Child namespaces
pub mod earth;
pub mod jupiter;
pub mod mars;
pub mod mercury;
pub mod neptune;
pub mod saturn;
pub mod uranus;
pub mod venus;

// === Types ===

/// Events from solar system observations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SolarEvent {
    /// Information about a celestial body
    Body {
body_type: BodyType,
mass_kg: f64,
name: String,
orbital_period_days: Option<f64>,
parent: Option<String>,
radius_km: f64,
    },
    /// System overview
    System {
moon_count: i64,
planet_count: i64,
star: String,
total_bodies: i64,
    },
}

/// Type of celestial body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyType {
    #[serde(rename = "star")]
    Star,
    #[serde(rename = "planet")]
    Planet,
    #[serde(rename = "dwarf_planet")]
    DwarfPlanet,
    #[serde(rename = "moon")]
    Moon,
}

// === Methods ===

/// Get information about a specific celestial body
pub async fn info(client: &PlexusClient, path: String) -> Result<Pin<Box<dyn Stream<Item = Result<SolarEvent>> + Send>>> {
    let stream = client.call_stream("solar.info", json!({ "path": path })).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<SolarEvent>(content) {
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
    client.call_single("solar.schema", serde_json::Value::Null).await
}

/// Observe the entire solar system
pub async fn observe(client: &PlexusClient) -> Result<Pin<Box<dyn Stream<Item = Result<SolarEvent>> + Send>>> {
    let stream = client.call_stream("solar.observe", serde_json::Value::Null).await?;

    // Filter and transform stream items to typed data
    let typed_stream = stream.filter_map(|item| async move {
        match item {
            Ok(PlexusStreamItem::Data { content, .. }) => {
                match serde_json::from_value::<SolarEvent>(content) {
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
