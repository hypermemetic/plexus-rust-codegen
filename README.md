# Substrate Rust Client

**Auto-generated Rust client for Substrate Plexus protocol.**

⚠️ **WARNING:** This repository is automatically generated. Do not edit files manually - changes will be overwritten on the next generation.

## Overview

This crate provides a fully-typed Rust client for interacting with Substrate via the Plexus protocol. It's automatically generated from the running Substrate instance's schema.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
substrate-client = { path = "../substrate-rust-codegen" }
# Or from git:
# substrate-client = { git = "https://github.com/your-org/substrate-rust-codegen" }
```

### Example

```rust
use substrate_client::PlexusClient;
use substrate_client::health;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to substrate
    let client = PlexusClient::new("ws://127.0.0.1:44410");

    // Call health check
    let status = health::check(&client).await?;
    println!("Health: {:?}", status);

    Ok(())
}
```

### Available Namespaces

The client is organized hierarchically matching Substrate's namespace structure:

```rust
use substrate_client::{
    health,           // Health checks
    cone,             // LLM conversation management
    arbor,            // Conversation tree storage
    bash,             // Command execution
    hyperforge::org,  // Multi-forge management
    // ... and more
};
```

### Streaming Methods

Methods that stream multiple events return `Stream<Item = Result<T>>`:

```rust
use futures::StreamExt;

let mut stream = cone::chat(&client, identifier, prompt).await?;

while let Some(event) = stream.next().await {
    match event? {
        // Handle streamed events
        _ => {}
    }
}
```

## Generation

This client is generated using the [plexus-codegen](../hub-codegen) pipeline:

```bash
cd ../plexus-codegen

# Auto-generate from running substrate
./scripts/update-rust-client.sh --generate ../substrate-rust-codegen

# Or with custom connection
./scripts/update-rust-client.sh --generate ../substrate-rust-codegen \
  --port 44410 \
  --backend substrate
```

## Versioning

Each generation is tagged with:
- **Timestamp**: When it was generated (UTC)
- **IR Hash**: Substrate configuration hash (first 8 chars)

Example tag: `v0.1.20260125135703-32e46854`

The version in `Cargo.toml` matches the tag for traceability:
```toml
version = "0.1.20260125135703-32e46854"
```

## Structure

The crate follows Substrate's hierarchical namespace structure:

```
src/
├── lib.rs              # Top-level re-exports
├── types.rs            # Core Plexus protocol types
├── client.rs           # WebSocket transport layer
├── health/mod.rs       # health.* methods
├── cone/mod.rs         # cone.* methods
├── arbor/mod.rs        # arbor.* methods
└── hyperforge/
    ├── mod.rs          # hyperforge.* methods
    ├── org/
    │   ├── mod.rs      # hyperforge.org.* methods
    │   └── hypermemetic/
    │       ├── mod.rs
    │       └── repos/mod.rs  # hyperforge.org.hypermemetic.repos.* methods
    └── ...
```

## Git History

Each update creates:
- **Commit** with detailed generation metadata
- **Git tag** with version number

Use `git log` to see generation history and `git tag -l` to list all versions.

## Dependencies

- `serde` - Serialization
- `serde_json` - JSON handling
- `tokio` - Async runtime
- `tokio-tungstenite` - WebSocket client
- `futures` - Stream utilities
- `anyhow` - Error handling
- `async-stream` - Stream helpers
- `thiserror` - Error types

## License

Same as Substrate project.

## Links

- **Substrate**: [../substrate](../substrate)
- **Plexus Codegen**: [../plexus-codegen](../plexus-codegen)
- **Synapse CLI**: [../synapse](../synapse)
