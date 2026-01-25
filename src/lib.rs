//! Auto-generated Plexus client
//! Do not edit manually

pub mod types;
pub mod client;

// Top-level namespace modules
pub mod arbor;
pub mod bash;
pub mod changelog;
pub mod claudecode;
pub mod cone;
pub mod echo;
pub mod health;
pub mod hyperforge;
pub mod jsexec;
pub mod loopback;
pub mod mustache;
pub mod solar;

pub use client::PlexusClient;

// Re-export common types
pub use types::PlexusStreamItem;