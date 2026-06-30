# modern-beta-api

Simple Rust client and bindings for the [Modern Beta](https://modernbeta.org) Minecraft server API.

## Installation

```toml
[dependencies]
modern-beta-api = { git = "https://github.com/zaknesler/modern-beta-rs" }
```

## Usage

```rust
use modern_beta_api::{Client, ClientConfig};

let client = Client::new(ClientConfig {
    api_key: "your-api-key".to_string(),
    ..Default::default()
})?;

// Player
client.get_player_profile("Karltroid").await?;
client.get_online_player_count().await?;
client.get_online_players().await?;

// Server
client.get_server_stats().await?;

// World
client.get_world().await?;
client.get_world_positions().await?;
```
