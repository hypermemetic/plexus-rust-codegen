//! Core transport types for Plexus protocol
//! Do not edit manually

use serde::{Deserialize, Serialize};

/// Metadata applied by the caller when wrapping activation responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    /// Call path through the system
    pub provenance: Vec<String>,
    /// Hash of plexus configuration for cache invalidation
    pub plexus_hash: String,
    /// Unix timestamp (seconds) when the event was wrapped
    pub timestamp: i64,
}

/// Universal stream item - all activations emit this type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PlexusStreamItem {
    /// Data payload with caller-applied metadata
    Data {
        /// Metadata from calling layer
        metadata: StreamMetadata,
        /// Type identifier for deserialization
        content_type: String,
        /// The actual payload (serialized activation event)
        content: serde_json::Value,
    },
    /// Progress update during long-running operations
    Progress {
        /// Metadata from calling layer
        metadata: StreamMetadata,
        /// Human-readable progress message
        message: String,
        /// Optional completion percentage (0.0 - 100.0)
        percentage: Option<f64>,
    },
    /// Error occurred during processing
    Error {
        /// Metadata from calling layer
        metadata: StreamMetadata,
        /// Human-readable error message
        message: String,
        /// Optional error code for programmatic handling
        code: Option<String>,
        /// Whether the operation can be retried
        recoverable: bool,
    },
    /// Stream completed successfully
    Done {
        /// Metadata from calling layer
        metadata: StreamMetadata,
    },
}

/// Error type for Plexus operations
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct PlexusError {
    pub message: String,
    pub code: Option<String>,
    pub recoverable: bool,
    pub metadata: Option<StreamMetadata>,
}
