//! Shared configuration structures for the test service
//! These are used by both the main service and the module generator

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[derive(Debug, Serialize, Deserialize, NixosType, PartialEq)]
pub struct ServerConfig {
    /// Enable the server
    #[nixos(default = "false")]
    pub enable: bool,

    /// Server port
    #[nixos(default = "8080")]
    pub port: u16,

    /// Server bind address
    #[nixos(default = "\"127.0.0.1\"")]
    pub bind_address: String,

    /// Maximum connections
    #[nixos(default = "100")]
    pub max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, NixosType, PartialEq)]
pub struct DatabaseConfig {
    /// Database host
    #[nixos(default = "\"localhost\"")]
    pub host: String,

    /// Database port
    #[nixos(default = "5432")]
    pub port: u16,

    /// Database name
    pub database: String,

    /// Enable SSL
    #[nixos(default = "false")]
    pub ssl: bool,
}

#[derive(Debug, Serialize, Deserialize, NixosType, PartialEq)]
pub struct TestServiceConfig {
    /// Service name
    pub service_name: String,

    /// Server configuration
    pub server: ServerConfig,

    /// Database configuration
    pub database: DatabaseConfig,

    /// Enable debug logging
    #[nixos(default = "false")]
    pub debug: bool,

    /// Log level
    #[nixos(default = "\"info\"")]
    pub log_level: String,
}
