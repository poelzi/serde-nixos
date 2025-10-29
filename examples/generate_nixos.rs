//! CLI tool example for generating NixOS modules from Rust configuration structures
//!
//! This example demonstrates how to create a CLI tool that reads configuration
//! from various sources and generates NixOS module files.

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct MyAppConfig {
    #[nixos(description = "Enable the MyApp service")]
    enable: bool,

    #[nixos(description = "Package to use for MyApp")]
    package: String,

    #[nixos(description = "User to run the service as", default = "\"myapp\"")]
    user: String,

    #[nixos(description = "Group to run the service as", default = "\"myapp\"")]
    group: String,

    #[nixos(
        description = "Working directory for the service",
        default = "\"/var/lib/myapp\""
    )]
    working_directory: String,

    #[nixos(description = "Server configuration")]
    server: ServerConfig,

    #[nixos(description = "Database configuration")]
    database: DatabaseConfig,

    #[nixos(description = "Logging configuration")]
    logging: LogConfig,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ServerConfig {
    #[nixos(description = "Port to listen on", default = "8080")]
    port: u16,

    #[nixos(description = "Address to bind to", default = "\"127.0.0.1\"")]
    bind_address: String,

    #[nixos(description = "Enable HTTPS")]
    enable_https: bool,

    #[nixos(description = "Path to SSL certificate")]
    ssl_cert: Option<String>,

    #[nixos(description = "Path to SSL key")]
    ssl_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct DatabaseConfig {
    #[nixos(description = "Database type (postgresql, mysql, sqlite)")]
    db_type: String,

    #[nixos(description = "Database connection URL")]
    url: String,

    #[nixos(
        description = "Maximum number of connections in the pool",
        default = "10"
    )]
    max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct LogConfig {
    #[nixos(
        description = "Log level (trace, debug, info, warn, error)",
        default = "\"info\""
    )]
    level: String,

    #[nixos(description = "Log to stdout")]
    stdout: bool,

    #[nixos(description = "Log file path")]
    file_path: Option<String>,

    #[nixos(description = "Log to syslog")]
    syslog: bool,
}

/// Command-line arguments (in a real CLI tool, you'd use clap or similar)
struct Args {
    /// Output file path
    output: Option<PathBuf>,
    /// Whether to include example configuration
    with_example: bool,
    /// Whether to generate a full NixOS module with systemd service
    full_module: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            output: None,
            with_example: true,
            full_module: true,
        }
    }
}

fn main() {
    let args = Args::default();

    println!("=== NixOS Module Generator for MyApp ===\n");

    if args.full_module {
        // Generate a complete NixOS module with systemd service
        let module = generate_full_nixos_module();

        if let Some(output_path) = args.output {
            fs::write(&output_path, module).expect("Failed to write module file");
            println!("Module written to: {:?}", output_path);
        } else {
            println!("Generated NixOS Module:\n");
            println!("{}", module);
        }
    } else {
        // Just generate the options definition
        let options = MyAppConfig::nixos_type_definition();

        if let Some(output_path) = args.output {
            fs::write(&output_path, options).expect("Failed to write options file");
            println!("Options written to: {:?}", output_path);
        } else {
            println!("Generated NixOS Options:\n");
            println!("{}", options);
        }
    }

    if args.with_example {
        println!("\n=== Example Configuration ===\n");
        let example_config = create_example_config();
        println!("{}", serde_json::to_string_pretty(&example_config).unwrap());

        println!("\n=== Example Nix Configuration ===\n");
        println!("{}", generate_example_nix_config());
    }
}

fn generate_full_nixos_module() -> String {
    let mut module = String::new();

    // Module header
    module.push_str("{ config, lib, pkgs, ... }:\n\n");
    module.push_str("with lib;\n\n");
    module.push_str("let\n");
    module.push_str("  cfg = config.services.myapp;\n");
    module.push_str("  configFile = pkgs.writeText \"myapp-config.json\" (builtins.toJSON cfg);\n");
    module.push_str("in\n");
    module.push_str("{\n");

    // Options section
    module.push_str("  options.services.myapp = {\n");

    // Get the generated options (without the module wrapper)
    let options = MyAppConfig::nixos_options();
    module.push_str(&indent_string(&options, 4));

    module.push_str("  };\n\n");

    // Config section with systemd service
    module.push_str("  config = mkIf cfg.enable {\n");

    // User and group
    module.push_str("    users.users.${cfg.user} = {\n");
    module.push_str("      isSystemUser = true;\n");
    module.push_str("      group = cfg.group;\n");
    module.push_str("      home = cfg.working_directory;\n");
    module.push_str("      createHome = true;\n");
    module.push_str("    };\n\n");

    module.push_str("    users.groups.${cfg.group} = {};\n\n");

    // Systemd service
    module.push_str("    systemd.services.myapp = {\n");
    module.push_str("      description = \"MyApp Service\";\n");
    module.push_str("      after = [ \"network.target\" ];\n");
    module.push_str("      wantedBy = [ \"multi-user.target\" ];\n\n");

    module.push_str("      serviceConfig = {\n");
    module.push_str("        Type = \"simple\";\n");
    module.push_str("        User = cfg.user;\n");
    module.push_str("        Group = cfg.group;\n");
    module.push_str("        WorkingDirectory = cfg.working_directory;\n");
    module.push_str("        ExecStart = \"${cfg.package}/bin/myapp --config ${configFile}\";\n");
    module.push_str("        Restart = \"on-failure\";\n");
    module.push_str("        RestartSec = \"5s\";\n");
    module.push_str("      };\n");
    module.push_str("    };\n");

    // Firewall rules if HTTPS is enabled
    module.push_str("\n    networking.firewall.allowedTCPPorts = optional cfg.server.enable_https cfg.server.port;\n");

    module.push_str("  };\n");
    module.push_str("}\n");

    module
}

fn create_example_config() -> MyAppConfig {
    MyAppConfig {
        enable: true,
        package: "pkgs.myapp".to_string(),
        user: "myapp".to_string(),
        group: "myapp".to_string(),
        working_directory: "/var/lib/myapp".to_string(),
        server: ServerConfig {
            port: 8080,
            bind_address: "0.0.0.0".to_string(),
            enable_https: false,
            ssl_cert: None,
            ssl_key: None,
        },
        database: DatabaseConfig {
            db_type: "postgresql".to_string(),
            url: "postgresql://myapp:password@localhost/myapp".to_string(),
            max_connections: 10,
        },
        logging: LogConfig {
            level: "info".to_string(),
            stdout: true,
            file_path: Some("/var/log/myapp/myapp.log".to_string()),
            syslog: false,
        },
    }
}

fn generate_example_nix_config() -> String {
    r#"services.myapp = {
  enable = true;
  package = pkgs.myapp;
  user = "myapp";
  group = "myapp";
  working_directory = "/var/lib/myapp";

  server = {
    port = 8080;
    bind_address = "0.0.0.0";
    enable_https = false;
  };

  database = {
    db_type = "postgresql";
    url = "postgresql://myapp:password@localhost/myapp";
    max_connections = 10;
  };

  logging = {
    level = "info";
    stdout = true;
    file_path = "/var/log/myapp/myapp.log";
    syslog = false;
  };
};"#
    .to_string()
}

fn indent_string(s: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    s.lines()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                format!("{}{}", indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
