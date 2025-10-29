//! Simple example demonstrating basic usage of serde-nixos

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ServerConfig {
    #[nixos(description = "Enable the server")]
    enable: bool,

    #[nixos(description = "The port number to listen on", default = "8080")]
    port: u16,

    #[nixos(
        description = "The hostname or IP address to bind to",
        default = "\"localhost\""
    )]
    host: String,

    #[nixos(description = "Maximum number of concurrent connections")]
    max_connections: Option<u32>,

    #[nixos(description = "Enable debug logging")]
    debug: bool,

    #[nixos(description = "List of allowed origins for CORS")]
    allowed_origins: Vec<String>,
}

fn main() {
    println!("=== Simple Server Configuration ===\n");

    // Generate the NixOS type definition
    let nixos_type_def = ServerConfig::nixos_type_definition();
    println!("NixOS Module Definition:");
    println!("{}", nixos_type_def);

    println!("\n=== Just the Options ===\n");
    let nixos_options = ServerConfig::nixos_options();
    println!("{}", nixos_options);

    println!("\n=== Type Expression ===\n");
    let nixos_type = ServerConfig::nixos_type();
    println!("{}", nixos_type);

    // Example of creating an actual config
    let config = ServerConfig {
        enable: true,
        port: 3000,
        host: "0.0.0.0".to_string(),
        max_connections: Some(100),
        debug: false,
        allowed_origins: vec![
            "http://localhost:3000".to_string(),
            "https://example.com".to_string(),
        ],
    };

    println!("\n=== Example Configuration (JSON) ===\n");
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
}
