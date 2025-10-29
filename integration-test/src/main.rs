//! Test service that validates NixOS-configured values

use std::env;
use std::fs;
use std::process;
use test_service_config::TestServiceConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <config-file>", args[0]);
        eprintln!("  or: {} --validate <config-file> <expected-json>", args[0]);
        process::exit(1);
    }

    // Check if we're in validate mode
    if args.len() >= 4 && args[1] == "--validate" {
        let config_path = &args[2];
        let expected_path = &args[3];

        let config_str = fs::read_to_string(config_path).unwrap_or_else(|e| {
            eprintln!("Failed to read config: {}", e);
            process::exit(1);
        });

        let expected_str = fs::read_to_string(expected_path).unwrap_or_else(|e| {
            eprintln!("Failed to read expected config: {}", e);
            process::exit(1);
        });

        let config: TestServiceConfig = serde_json::from_str(&config_str).unwrap_or_else(|e| {
            eprintln!("Failed to parse config: {}", e);
            process::exit(1);
        });

        let expected: TestServiceConfig = serde_json::from_str(&expected_str).unwrap_or_else(|e| {
            eprintln!("Failed to parse expected config: {}", e);
            process::exit(1);
        });

        if config == expected {
            println!("✓ Configuration validation PASSED");
            println!("All values match expected configuration:");
            println!("  Service: {}", config.service_name);
            println!(
                "  Server: {}:{}",
                config.server.bind_address, config.server.port
            );
            println!(
                "  Database: {}:{}/{}",
                config.database.host, config.database.port, config.database.database
            );
            println!("  Debug: {}", config.debug);
            println!("  Log Level: {}", config.log_level);
            process::exit(0);
        } else {
            eprintln!("✗ Configuration validation FAILED");
            eprintln!("\nExpected:");
            eprintln!("{}", serde_json::to_string_pretty(&expected).unwrap());
            eprintln!("\nActual:");
            eprintln!("{}", serde_json::to_string_pretty(&config).unwrap());
            process::exit(1);
        }
    }

    // Normal mode: run the service
    let config_path = &args[1];

    let config_str = fs::read_to_string(config_path).unwrap_or_else(|e| {
        eprintln!("Failed to read config file {}: {}", config_path, e);
        process::exit(1);
    });

    let config: TestServiceConfig = serde_json::from_str(&config_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse config: {}", e);
        process::exit(1);
    });

    println!("Test Service Starting");
    println!("===================");
    println!("Service Name: {}", config.service_name);
    println!("\nServer Configuration:");
    println!("  Enable: {}", config.server.enable);
    println!(
        "  Address: {}:{}",
        config.server.bind_address, config.server.port
    );
    println!("  Max Connections: {}", config.server.max_connections);
    println!("\nDatabase Configuration:");
    println!("  Host: {}", config.database.host);
    println!("  Port: {}", config.database.port);
    println!("  Database: {}", config.database.database);
    println!("  SSL: {}", config.database.ssl);
    println!("\nLogging:");
    println!("  Debug: {}", config.debug);
    println!("  Level: {}", config.log_level);
    println!("\n✓ Configuration loaded successfully");
    println!("Service would run here (test mode - exiting)");
}
