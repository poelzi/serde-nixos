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

#[test]
fn test_enum_serde_rename_all() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "kebab-case")]
    enum ServiceMode {
        DryRun,
        FullSync,
    }

    let nixos_type = ServiceMode::nixos_type();

    assert!(nixos_type.contains("\"dry-run\""));
    assert!(nixos_type.contains("\"full-sync\""));
}

#[test]
fn test_enum_variant_serde_rename() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "snake_case")]
    enum Role {
        Admin,
        #[serde(rename = "read-only")]
        ReadOnly,
    }

    let nixos_type = Role::nixos_type();

    assert!(nixos_type.contains("\"admin\""));
    assert!(nixos_type.contains("\"read-only\""));
}

#[test]
fn test_enum_rename_all_lowercase() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "lowercase")]
    enum Color {
        Red,
        DarkBlue,
    }

    let nixos_type = Color::nixos_type();

    // Serde lowercase on PascalCase variants: no separators
    assert!(nixos_type.contains("\"red\""));
    assert!(nixos_type.contains("\"darkblue\""));
}

#[test]
fn test_enum_rename_all_uppercase() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "UPPERCASE")]
    enum Size {
        Small,
        ExtraLarge,
    }

    let nixos_type = Size::nixos_type();

    // Serde UPPERCASE on PascalCase variants: no separators
    assert!(nixos_type.contains("\"SMALL\""));
    assert!(nixos_type.contains("\"EXTRALARGE\""));
}

#[test]
fn test_enum_rename_all_screaming_snake() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    enum Priority {
        Low,
        VeryHigh,
    }

    let nixos_type = Priority::nixos_type();

    assert!(nixos_type.contains("\"LOW\""));
    assert!(nixos_type.contains("\"VERY_HIGH\""));
}
