//! # serde-nixos
//!
//! Generate NixOS type definitions from your Rust structures.
//!
//! This crate provides a derive macro that automatically generates NixOS module
//! definitions from your Rust configuration structures, ensuring type safety
//! across both Rust and Nix.
//!
//! ## Example
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_nixos::NixosType;
//!
//! #[derive(Serialize, Deserialize, NixosType)]
//! struct ServerConfig {
//!     #[nixos(description = "The server port to listen on")]
//!     port: u16,
//!
//!     #[nixos(default = "\"localhost\"", description = "The hostname to bind to")]
//!     host: String,
//!
//!     #[nixos(description = "Maximum number of concurrent connections")]
//!     max_connections: Option<u32>,
//!
//!     #[nixos(description = "Database configuration")]
//!     database: DatabaseConfig,
//! }
//!
//! #[derive(Serialize, Deserialize, NixosType)]
//! struct DatabaseConfig {
//!     #[nixos(description = "Database connection string")]
//!     url: String,
//!
//!     #[nixos(default = "5", description = "Connection pool size")]
//!     pool_size: u32,
//! }
//! ```
//!
//! Then generate the NixOS module:
//!
//! ```rust
//! # use serde::{Serialize, Deserialize};
//! # use serde_nixos::NixosType;
//! #
//! # #[derive(Serialize, Deserialize, NixosType)]
//! # struct ServerConfig {
//! #     enable: bool,
//! # }
//! #
//! let nixos_module = ServerConfig::nixos_type_definition();
//! println!("{}", nixos_module);
//! ```
//!
//! This will output a NixOS module definition that can be used in your NixOS configuration.

pub use serde_nixos_macros::{nixos_module, NixosType};

/// Re-export commonly used serde traits for convenience
pub use serde::{Deserialize, Serialize};

/// Generator utilities for building NixOS modules
pub mod generator;

/// Helper trait for types that can generate NixOS definitions
pub trait NixosTypeGenerator {
    /// Generate a complete NixOS module definition
    fn nixos_type_definition() -> String;

    /// Generate just the options portion of the module
    fn nixos_options() -> String;

    /// Get the NixOS type expression for this type
    fn nixos_type() -> String;
}

/// Utility functions for working with NixOS types
pub mod utils {

    /// Format a Rust value as a Nix expression
    pub fn format_nix_value(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => format!("\"{}\"", escape_nix_string(s)),
            serde_json::Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(format_nix_value).collect();
                format!("[ {} ]", items.join(" "))
            }
            serde_json::Value::Object(obj) => {
                let attrs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{} = {};", k, format_nix_value(v)))
                    .collect();
                format!("{{ {} }}", attrs.join(" "))
            }
        }
    }

    /// Escape a string for use in Nix expressions
    pub fn escape_nix_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }

    /// Generate a NixOS module file with proper formatting
    pub fn generate_module_file(module_name: &str, options: &str, config: Option<&str>) -> String {
        let mut result = String::new();

        result.push_str("{ config, lib, pkgs, ... }:\n\n");
        result.push_str("with lib;\n\n");
        result.push_str("{\n");

        // Add options
        result.push_str("  options.");
        result.push_str(module_name);
        result.push_str(" = {\n");
        result.push_str(options);
        result.push_str("  };\n\n");

        // Add config if provided
        if let Some(cfg) = config {
            result.push_str("  config = mkIf config.");
            result.push_str(module_name);
            result.push_str(".enable {\n");
            result.push_str(cfg);
            result.push_str("  };\n");
        }

        result.push_str("}\n");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_value_formatting() {
        assert_eq!(utils::format_nix_value(&serde_json::json!(null)), "null");
        assert_eq!(utils::format_nix_value(&serde_json::json!(true)), "true");
        assert_eq!(utils::format_nix_value(&serde_json::json!(42)), "42");
        assert_eq!(
            utils::format_nix_value(&serde_json::json!("hello")),
            "\"hello\""
        );
        assert_eq!(
            utils::format_nix_value(&serde_json::json!([1, 2, 3])),
            "[ 1 2 3 ]"
        );
    }

    #[test]
    fn test_escape_nix_string() {
        assert_eq!(utils::escape_nix_string("hello"), "hello");
        assert_eq!(utils::escape_nix_string("hello\"world"), "hello\\\"world");
        assert_eq!(utils::escape_nix_string("line1\nline2"), "line1\\nline2");
    }
}
