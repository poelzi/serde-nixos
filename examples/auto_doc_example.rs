//! Example demonstrating the #[nixos(auto_doc)] feature
//!
//! This shows how to automatically use doc comments as NixOS option descriptions
//! without needing to specify #[nixos(description = "...")] on every field.

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

/// Traditional approach: explicit descriptions
#[derive(Debug, Serialize, Deserialize, NixosType)]
struct TraditionalConfig {
    #[nixos(description = "The server port")]
    port: u16,

    #[nixos(description = "The bind address")]
    bind_address: String,

    #[nixos(description = "Enable debug mode")]
    debug: bool,
}

/// Modern approach: auto_doc enabled - just use doc comments!
#[derive(Debug, Serialize, Deserialize, NixosType)]
#[nixos(auto_doc)]
struct ModernConfig {
    /// The server port
    #[nixos(default = "8080")]
    port: u16,

    /// The bind address
    #[nixos(default = "\"127.0.0.1\"")]
    bind_address: String,

    /// Enable debug mode
    #[nixos(default = "false")]
    debug: bool,

    /// Maximum number of connections
    ///
    /// This can be set to limit resource usage.
    /// Multi-line doc comments are supported!
    #[nixos(default = "100")]
    max_connections: u32,
}

/// Database configuration with auto_doc
#[derive(Debug, Serialize, Deserialize, NixosType)]
#[nixos(auto_doc)]
struct DatabaseConfig {
    /// Database host address
    #[nixos(default = "\"localhost\"")]
    host: String,

    /// Database port number
    #[nixos(default = "5432")]
    port: u16,

    /// Name of the database to connect to
    database: String,

    /// Enable SSL/TLS for the connection
    #[nixos(default = "true")]
    ssl: bool,

    /// Connection timeout in seconds
    #[nixos(default = "30")]
    timeout: u32,
}

/// Application configuration with nested auto_doc structs
#[derive(Debug, Serialize, Deserialize, NixosType)]
#[nixos(auto_doc)]
struct ApplicationConfig {
    /// Name of the application
    app_name: String,

    /// Version string
    version: String,

    /// Server configuration
    server: ModernConfig,

    /// Database configuration
    database: DatabaseConfig,

    /// Enable verbose logging
    #[nixos(default = "false")]
    verbose: bool,

    /// Log file path
    ///
    /// If not specified, logs will be written to stdout
    #[nixos(default = "null")]
    log_file: Option<String>,
}

fn main() {
    println!("=== Auto-Doc Feature Example ===\n");

    println!("Traditional approach (explicit descriptions):");
    println!("{}", "=".repeat(60));
    let traditional = TraditionalConfig::nixos_options();
    println!("{}\n", traditional);

    println!("Modern approach (#[nixos(auto_doc)]):");
    println!("{}", "=".repeat(60));
    let modern = ModernConfig::nixos_options();
    println!("{}\n", modern);

    println!("Notice how both generate the same descriptions!");
    println!("But with auto_doc, you don't need #[nixos(description = \"...\")] on every field.\n");

    println!("Database Configuration (with auto_doc):");
    println!("{}", "=".repeat(60));
    println!("{}\n", DatabaseConfig::nixos_type_definition());

    println!("Complete Application with Nested Configs:");
    println!("{}", "=".repeat(60));
    println!("{}\n", ApplicationConfig::nixos_type_definition());

    println!("Full Definition (let chain):");
    println!("{}", "=".repeat(60));
    println!("{}\n", ApplicationConfig::nixos_type_full_definition());

    // Show an example configuration
    let config = ApplicationConfig {
        app_name: "MyApp".to_string(),
        version: "1.0.0".to_string(),
        server: ModernConfig {
            port: 8080,
            bind_address: "0.0.0.0".to_string(),
            debug: false,
            max_connections: 200,
        },
        database: DatabaseConfig {
            host: "db.example.com".to_string(),
            port: 5432,
            database: "myapp".to_string(),
            ssl: true,
            timeout: 30,
        },
        verbose: false,
        log_file: Some("/var/log/myapp.log".to_string()),
    };

    println!("Example Configuration (JSON):");
    println!("{}", "=".repeat(60));
    println!("{}\n", serde_json::to_string_pretty(&config).unwrap());

    println!("Benefits of auto_doc:");
    println!("  ✓ Less repetition - doc comments serve dual purpose");
    println!("  ✓ Keeps docs close to code - easier to maintain");
    println!("  ✓ Standard Rust documentation appears in NixOS options");
    println!("  ✓ Can still override with #[nixos(description)] when needed");
    println!("  ✓ Multi-line doc comments work perfectly");
}
