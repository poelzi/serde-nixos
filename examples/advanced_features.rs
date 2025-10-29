//! Advanced example demonstrating all the new features:
//! - Doc comment extraction for descriptions
//! - All mkOption attributes
//! - Named type definitions
//! - Full definition with let chains

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ResourceLimits {
    /// Maximum CPU cores to allocate
    #[nixos(default = "2")]
    cpu_cores: u32,

    /// Memory limit in megabytes
    #[nixos(default = "1024", example = "2048")]
    memory_mb: u64,

    /// Disk space quota in gigabytes
    #[nixos(default = "10")]
    disk_gb: u64,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct NetworkConfig {
    /// Enable IPv4 networking
    #[nixos(default = "true")]
    enable_ipv4: bool,

    /// Enable IPv6 networking
    #[nixos(default = "false")]
    enable_ipv6: bool,

    /// Port number for the main service
    #[nixos(default = "8080", example = "3000")]
    port: u16,

    /// Bind address for the service
    #[nixos(default = "\"127.0.0.1\"")]
    bind_address: String,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ServiceConfig {
    /// Enable the service
    #[nixos(default = "false")]
    enable: bool,

    /// Package to use for this service
    #[nixos(
        example = "pkgs.myservice",
        description = "The package containing the service binary"
    )]
    package: String,

    /// User to run the service as
    #[nixos(default = "\"myservice\"", apply = "x: assert x != \"root\"; x")]
    user: String,

    /// Network configuration
    network: NetworkConfig,

    /// Resource limits for the service
    resource_limits: ResourceLimits,

    /// Advanced options (internal use only)
    #[nixos(internal)]
    internal_state: Option<String>,

    /// Read-only configuration checksum
    #[nixos(read_only, example = "\"abc123\"")]
    config_checksum: Option<String>,

    /// Environment variables
    #[nixos(default = "{}", example = "{ FOO = \"bar\"; BAZ = \"qux\"; }")]
    environment: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct AdvancedServiceConfig {
    /// Main service configuration
    service: ServiceConfig,

    /// Enable monitoring
    #[nixos(default = "true", description = "Enable Prometheus metrics endpoint")]
    enable_monitoring: bool,

    /// Monitoring port
    #[nixos(default = "9090")]
    monitoring_port: u16,

    /// List of additional features to enable
    #[nixos(default = "[]", example = "[\"feature-a\" \"feature-b\"]")]
    features: Vec<String>,

    /// Log level configuration
    #[nixos(
        default = "\"info\"",
        example = "\"debug\"",
        apply = "x: lib.toLower x"
    )]
    log_level: String,
}

fn main() {
    println!("=== Advanced Features Demo ===\n");

    // Show the named type for ResourceLimits
    println!(
        "Type name for ResourceLimits: {}",
        ResourceLimits::nixos_type_name()
    );
    println!();

    // Show the type definition with named type
    println!("=== ResourceLimits Type Definition ===");
    println!("{}", ResourceLimits::nixos_type_definition());
    println!();

    // Show NetworkConfig type definition
    println!("=== NetworkConfig Type Definition ===");
    println!("{}", NetworkConfig::nixos_type_definition());
    println!();

    // Show ServiceConfig with all its nested types
    println!("=== ServiceConfig Type Definition ===");
    println!("{}", ServiceConfig::nixos_type_definition());
    println!();

    // Show the full definition with let chain
    println!("=== ServiceConfig Full Definition (with let chain) ===");
    println!("{}", ServiceConfig::nixos_type_full_definition());
    println!();

    // Show the top-level service with all nested types
    println!("=== AdvancedServiceConfig Full Definition ===");
    println!("{}", AdvancedServiceConfig::nixos_type_full_definition());
    println!();

    // Show how the type name is used
    println!("=== Using Type Names ===");
    println!("ServiceConfig type: {}", ServiceConfig::nixos_type());
    println!("ResourceLimits type: {}", ResourceLimits::nixos_type());
    println!();

    // Show options with all mkOption attributes
    println!("=== ServiceConfig Options (showing all mkOption attributes) ===");
    println!("{}", ServiceConfig::nixos_options());
    println!();

    // Example configuration
    println!("=== Example Configuration ===");
    let config = AdvancedServiceConfig {
        service: ServiceConfig {
            enable: true,
            package: "pkgs.myservice".to_string(),
            user: "myservice".to_string(),
            network: NetworkConfig {
                enable_ipv4: true,
                enable_ipv6: false,
                port: 8080,
                bind_address: "0.0.0.0".to_string(),
            },
            resource_limits: ResourceLimits {
                cpu_cores: 4,
                memory_mb: 2048,
                disk_gb: 20,
            },
            internal_state: None,
            config_checksum: Some("abc123def456".to_string()),
            environment: {
                let mut env = std::collections::HashMap::new();
                env.insert("LOG_FORMAT".to_string(), "json".to_string());
                env.insert("RUST_LOG".to_string(), "info".to_string());
                env
            },
        },
        enable_monitoring: true,
        monitoring_port: 9090,
        features: vec!["metrics".to_string(), "tracing".to_string()],
        log_level: "info".to_string(),
    };

    println!("{}", serde_json::to_string_pretty(&config).unwrap());

    println!("\n=== Usage in NixOS Configuration ===");
    println!(
        r#"
services.myAdvancedService = {{
  service = {{
    enable = true;
    package = pkgs.myservice;
    user = "myservice";

    network = {{
      enable_ipv4 = true;
      enable_ipv6 = false;
      port = 8080;
      bind_address = "0.0.0.0";
    }};

    resource_limits = {{
      cpu_cores = 4;
      memory_mb = 2048;
      disk_gb = 20;
    }};

    environment = {{
      LOG_FORMAT = "json";
      RUST_LOG = "info";
    }};
  }};

  enable_monitoring = true;
  monitoring_port = 9090;
  features = [ "metrics" "tracing" ];
  log_level = "info";
}};
"#
    );
}
