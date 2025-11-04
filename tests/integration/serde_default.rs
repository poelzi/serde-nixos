//! Test for serde(default) attribute handling
//!
//! This tests the bug fix for parsing #[serde(default = "function")] pattern

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

// Default functions
fn default_port() -> u16 {
    8080
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_max_connections() -> u32 {
    100
}

fn default_enabled() -> bool {
    true
}

/// Test struct with various serde(default) patterns
#[derive(Debug, Clone, Serialize, Deserialize, NixosType)]
struct ServerConfig {
    /// Port with default function
    #[serde(default = "default_port")]
    pub port: u16,

    /// Host with default function
    #[serde(default = "default_host")]
    pub host: String,

    /// Max connections with default function
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Enabled with bare default (uses Default trait)
    #[serde(default)]
    pub debug: bool,

    /// Enabled with default function
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[test]
fn test_serde_default_with_function() {
    // Should compile and generate NixOS type without errors
    let nixos_type = ServerConfig::nixos_type();

    // Verify it generates valid output
    assert!(!nixos_type.is_empty());

    // The main test is that this compiles without "expected `,`" errors
    // which was the bug we fixed
}

#[test]
fn test_serde_default_function_parsing() {
    // This test verifies that the macro correctly parses default = "function"
    // without syntax errors - this is the core bug fix test

    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct TestConfig {
        #[serde(default = "default_port")]
        port: u16,
    }

    // If this compiles without errors, the bug is fixed
    let nixos_type = TestConfig::nixos_type();
    assert!(!nixos_type.is_empty());
}

#[test]
fn test_mixed_default_patterns() {
    // Test mixing bare default and function default

    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct MixedConfig {
        #[serde(default)]
        flag1: bool,

        #[serde(default = "default_enabled")]
        flag2: bool,

        #[serde(default)]
        count: i32,

        #[serde(default = "default_max_connections")]
        max: u32,
    }

    // The bug was a parse error on default = "function"
    // If this compiles, the fix works
    let nixos_type = MixedConfig::nixos_type();
    assert!(!nixos_type.is_empty());
}

#[test]
fn test_default_with_nested_structs() {
    #[derive(Debug, Default, Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        #[serde(default = "default_host")]
        host: String,

        #[serde(default = "default_port")]
        port: u16,
    }

    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct AppConfig {
        #[serde(default)]
        database: DatabaseConfig,

        #[serde(default = "default_enabled")]
        enabled: bool,
    }

    // Test nested structs with default functions
    let nixos_type = AppConfig::nixos_type();
    assert!(!nixos_type.is_empty());
}

#[test]
fn test_multiple_defaults_in_one_struct() {
    // Regression test: multiple fields with default = "function" in same struct

    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct MultiDefaultConfig {
        #[serde(default = "default_host")]
        host: String,

        #[serde(default = "default_port")]
        port: u16,

        #[serde(default = "default_max_connections")]
        max_conn: u32,

        #[serde(default = "default_enabled")]
        enabled: bool,
    }

    // Before the fix, this would fail with "expected `,`" on each default = "..."
    let nixos_type = MultiDefaultConfig::nixos_type();
    assert!(!nixos_type.is_empty());
}
