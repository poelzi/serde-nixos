use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_simple_enum() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum LogLevel {
        Trace,
        Debug,
        Info,
        Warn,
        Error,
    }

    let nixos_type = LogLevel::nixos_type();

    // Should generate types.enum with all variants
    assert!(nixos_type.contains("types.enum"));
    assert!(nixos_type.contains("\"Trace\""));
    assert!(nixos_type.contains("\"Debug\""));
    assert!(nixos_type.contains("\"Info\""));
    assert!(nixos_type.contains("\"Warn\""));
    assert!(nixos_type.contains("\"Error\""));
}

#[test]
fn test_enum_in_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum DatabaseType {
        PostgreSQL,
        MySQL,
        SQLite,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        #[nixos(description = "Database type to use")]
        db_type: DatabaseType,

        #[nixos(description = "Connection string")]
        connection: String,
    }

    // We can't easily test the nested enum type generation
    // without more complex macro expansion, but we can verify
    // that the struct generates properly
    let options = DatabaseConfig::nixos_options();
    assert!(options.contains("db_type = lib.mkOption"));
    assert!(options.contains("connection = lib.mkOption"));
}

#[test]
fn test_optional_enum() {
    #[derive(Serialize, Deserialize, NixosType)]
    enum Protocol {
        Http,
        Https,
        Ws,
        Wss,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct ServerConfig {
        protocol: Option<Protocol>,
    }

    let options = ServerConfig::nixos_options();
    assert!(options.contains("protocol = lib.mkOption"));
    // Should wrap enum in nullOr
    assert!(options.contains("types.nullOr"));
}
