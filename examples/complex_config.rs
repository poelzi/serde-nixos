//! Complex example with nested structures and various data types

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ApplicationConfig {
    #[nixos(description = "Enable the application")]
    enable: bool,

    #[nixos(description = "Application name")]
    name: String,

    #[nixos(description = "Application version")]
    version: String,

    #[nixos(description = "Server configuration")]
    server: ServerConfig,

    #[nixos(description = "Database configuration")]
    database: DatabaseConfig,

    #[nixos(description = "Logging configuration")]
    logging: LoggingConfig,

    #[nixos(description = "Feature flags")]
    features: HashMap<String, bool>,

    #[nixos(description = "List of plugins to load")]
    plugins: Vec<PluginConfig>,

    #[nixos(description = "Optional monitoring configuration")]
    monitoring: Option<MonitoringConfig>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ServerConfig {
    #[nixos(description = "Server bind address", default = "\"0.0.0.0\"")]
    bind_address: String,

    #[nixos(description = "Server port", default = "8080")]
    port: u16,

    #[nixos(description = "Use HTTPS")]
    use_https: bool,

    #[nixos(description = "SSL certificate path")]
    cert_path: Option<String>,

    #[nixos(description = "SSL key path")]
    key_path: Option<String>,

    #[nixos(description = "Request timeout in seconds", default = "30")]
    request_timeout: u32,

    #[nixos(
        description = "Maximum request body size in bytes",
        default = "10485760"
    )]
    max_body_size: u64,

    #[nixos(description = "Worker threads", default = "4")]
    worker_threads: u32,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct DatabaseConfig {
    #[nixos(description = "Database type (postgresql, mysql, sqlite)")]
    db_type: String,

    #[nixos(description = "Database host", default = "\"localhost\"")]
    host: String,

    #[nixos(description = "Database port", default = "5432")]
    port: u16,

    #[nixos(description = "Database name")]
    database: String,

    #[nixos(description = "Database username")]
    username: String,

    #[nixos(description = "Database password (consider using secrets management)")]
    password: String,

    #[nixos(description = "Connection pool configuration")]
    pool: ConnectionPoolConfig,

    #[nixos(description = "Enable SSL for database connection")]
    ssl_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct ConnectionPoolConfig {
    #[nixos(description = "Minimum number of connections", default = "2")]
    min_connections: u32,

    #[nixos(description = "Maximum number of connections", default = "10")]
    max_connections: u32,

    #[nixos(description = "Connection timeout in seconds", default = "30")]
    connection_timeout: u32,

    #[nixos(description = "Idle timeout in seconds", default = "600")]
    idle_timeout: u32,

    #[nixos(description = "Maximum connection lifetime in seconds")]
    max_lifetime: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct LoggingConfig {
    #[nixos(description = "Log level (trace, debug, info, warn, error)")]
    level: String,

    #[nixos(description = "Log format (json, plain, pretty)")]
    format: String,

    #[nixos(description = "Log outputs")]
    outputs: Vec<LogOutput>,

    #[nixos(description = "Enable structured logging")]
    structured: bool,

    #[nixos(description = "Include timestamps in logs")]
    timestamps: bool,

    #[nixos(description = "Log rotation configuration")]
    rotation: Option<LogRotationConfig>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct LogOutput {
    #[nixos(description = "Output type (stdout, stderr, file, syslog)")]
    output_type: String,

    #[nixos(description = "File path for file output")]
    path: Option<String>,

    #[nixos(description = "Minimum log level for this output")]
    min_level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct LogRotationConfig {
    #[nixos(
        description = "Maximum file size in bytes before rotation",
        default = "104857600"
    )]
    max_size: u64,

    #[nixos(description = "Maximum number of backup files to keep", default = "10")]
    max_backups: u32,

    #[nixos(description = "Maximum age of log files in days", default = "30")]
    max_age: u32,

    #[nixos(description = "Compress rotated files")]
    compress: bool,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct PluginConfig {
    #[nixos(description = "Plugin name")]
    name: String,

    #[nixos(description = "Plugin enabled state")]
    enabled: bool,

    #[nixos(description = "Plugin configuration as key-value pairs")]
    config: HashMap<String, serde_json::Value>,

    #[nixos(description = "Plugin dependencies")]
    dependencies: Vec<String>,

    #[nixos(
        description = "Plugin load priority (lower numbers load first)",
        default = "100"
    )]
    priority: i32,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct MonitoringConfig {
    #[nixos(description = "Enable metrics collection")]
    metrics_enabled: bool,

    #[nixos(description = "Metrics endpoint path", default = "\"/metrics\"")]
    metrics_path: String,

    #[nixos(description = "Enable health checks")]
    health_check_enabled: bool,

    #[nixos(description = "Health check endpoint path", default = "\"/health\"")]
    health_check_path: String,

    #[nixos(description = "Prometheus configuration")]
    prometheus: Option<PrometheusConfig>,

    #[nixos(description = "OpenTelemetry configuration")]
    opentelemetry: Option<OpenTelemetryConfig>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct PrometheusConfig {
    #[nixos(description = "Prometheus pushgateway URL")]
    pushgateway_url: Option<String>,

    #[nixos(description = "Push interval in seconds", default = "10")]
    push_interval: u32,

    #[nixos(description = "Job label for metrics")]
    job_label: String,

    #[nixos(description = "Additional labels")]
    labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, NixosType)]
struct OpenTelemetryConfig {
    #[nixos(description = "OpenTelemetry collector endpoint")]
    endpoint: String,

    #[nixos(description = "Service name")]
    service_name: String,

    #[nixos(description = "Enable tracing")]
    tracing_enabled: bool,

    #[nixos(description = "Enable metrics")]
    metrics_enabled: bool,

    #[nixos(description = "Sampling rate (0.0 to 1.0)", default = "1.0")]
    sampling_rate: f64,

    #[nixos(description = "Export timeout in seconds", default = "30")]
    export_timeout: u32,
}

fn main() {
    println!("=== Complex Application Configuration ===\n");

    // Generate the NixOS type definition for the main config
    let nixos_type_def = ApplicationConfig::nixos_type_definition();
    println!("Main Application NixOS Module Definition:");
    println!("{}", nixos_type_def);

    println!("\n{}\n", "=".repeat(60));

    // Also show nested type definitions
    println!("Database Configuration Module:");
    let db_nixos = DatabaseConfig::nixos_type_definition();
    println!("{}", db_nixos);

    println!("\n{}\n", "=".repeat(60));

    println!("Monitoring Configuration Module:");
    let monitoring_nixos = MonitoringConfig::nixos_type_definition();
    println!("{}", monitoring_nixos);

    // Create an example configuration
    let config = ApplicationConfig {
        enable: true,
        name: "my-awesome-app".to_string(),
        version: "1.0.0".to_string(),
        server: ServerConfig {
            bind_address: "0.0.0.0".to_string(),
            port: 8080,
            use_https: true,
            cert_path: Some("/etc/ssl/certs/server.crt".to_string()),
            key_path: Some("/etc/ssl/private/server.key".to_string()),
            request_timeout: 30,
            max_body_size: 10_485_760,
            worker_threads: 4,
        },
        database: DatabaseConfig {
            db_type: "postgresql".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "myapp".to_string(),
            username: "appuser".to_string(),
            password: "secret".to_string(),
            pool: ConnectionPoolConfig {
                min_connections: 2,
                max_connections: 10,
                connection_timeout: 30,
                idle_timeout: 600,
                max_lifetime: Some(3600),
            },
            ssl_mode: true,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            format: "json".to_string(),
            outputs: vec![
                LogOutput {
                    output_type: "stdout".to_string(),
                    path: None,
                    min_level: Some("info".to_string()),
                },
                LogOutput {
                    output_type: "file".to_string(),
                    path: Some("/var/log/myapp/app.log".to_string()),
                    min_level: Some("debug".to_string()),
                },
            ],
            structured: true,
            timestamps: true,
            rotation: Some(LogRotationConfig {
                max_size: 104_857_600,
                max_backups: 10,
                max_age: 30,
                compress: true,
            }),
        },
        features: {
            let mut features = HashMap::new();
            features.insert("experimental_api".to_string(), false);
            features.insert("analytics".to_string(), true);
            features.insert("cache".to_string(), true);
            features
        },
        plugins: vec![
            PluginConfig {
                name: "auth".to_string(),
                enabled: true,
                config: {
                    let mut config = HashMap::new();
                    config.insert("provider".to_string(), serde_json::json!("oauth2"));
                    config.insert("client_id".to_string(), serde_json::json!("my-client-id"));
                    config
                },
                dependencies: vec![],
                priority: 10,
            },
            PluginConfig {
                name: "rate_limiter".to_string(),
                enabled: true,
                config: {
                    let mut config = HashMap::new();
                    config.insert("max_requests".to_string(), serde_json::json!(100));
                    config.insert("window_seconds".to_string(), serde_json::json!(60));
                    config
                },
                dependencies: vec!["auth".to_string()],
                priority: 20,
            },
        ],
        monitoring: Some(MonitoringConfig {
            metrics_enabled: true,
            metrics_path: "/metrics".to_string(),
            health_check_enabled: true,
            health_check_path: "/health".to_string(),
            prometheus: Some(PrometheusConfig {
                pushgateway_url: Some("http://prometheus-pushgateway:9091".to_string()),
                push_interval: 10,
                job_label: "myapp".to_string(),
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("environment".to_string(), "production".to_string());
                    labels.insert("region".to_string(), "us-west-2".to_string());
                    labels
                },
            }),
            opentelemetry: Some(OpenTelemetryConfig {
                endpoint: "http://otel-collector:4317".to_string(),
                service_name: "myapp".to_string(),
                tracing_enabled: true,
                metrics_enabled: true,
                sampling_rate: 1.0,
                export_timeout: 30,
            }),
        }),
    };

    println!("\n=== Example Configuration (JSON) ===\n");
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
}
