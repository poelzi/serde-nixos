use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_full_definition_single_nested_type() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        #[nixos(description = "Database host")]
        host: String,

        #[nixos(description = "Database port", default = "5432")]
        port: u16,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct AppConfig {
        #[nixos(description = "App name")]
        name: String,

        #[nixos(description = "Database configuration")]
        database: DatabaseConfig,
    }

    let full_def = AppConfig::nixos_type_full_definition();

    // Verify structure
    assert!(full_def.contains("let"), "Should start with 'let'");
    assert!(
        full_def.contains("in appConfigType"),
        "Should end with 'in appConfigType'"
    );

    // Verify databaseConfigType is defined
    assert!(
        full_def.contains("databaseConfigType = types.submodule"),
        "Should define databaseConfigType"
    );
    assert!(
        full_def.contains("host = lib.mkOption"),
        "databaseConfigType should have host option"
    );
    assert!(
        full_def.contains("port = lib.mkOption"),
        "databaseConfigType should have port option"
    );

    // Verify appConfigType is defined
    assert!(
        full_def.contains("appConfigType = types.submodule"),
        "Should define appConfigType"
    );

    // Verify appConfigType references databaseConfigType
    assert!(
        full_def.contains("type = databaseConfigType")
            || full_def.contains("type = types.nullOr databaseConfigType"),
        "appConfigType should reference databaseConfigType"
    );

    // Verify dependency order
    let db_pos = full_def
        .find("databaseConfigType = types.submodule")
        .expect("databaseConfigType not found");
    let app_pos = full_def
        .find("appConfigType = types.submodule")
        .expect("appConfigType not found");
    assert!(
        db_pos < app_pos,
        "databaseConfigType should come before appConfigType"
    );
}

#[test]
fn test_full_definition_multiple_nested_types() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct ServerConfig {
        #[nixos(description = "Port", default = "8080")]
        port: u16,

        #[nixos(description = "Enable SSL")]
        ssl: bool,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        #[nixos(description = "Host")]
        host: String,

        #[nixos(description = "Port", default = "5432")]
        port: u16,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct AppConfig {
        #[nixos(description = "Application name")]
        name: String,

        #[nixos(description = "Server config")]
        server: ServerConfig,

        #[nixos(description = "Database config")]
        database: DatabaseConfig,
    }

    let full_def = AppConfig::nixos_type_full_definition();

    // All three types should be defined
    assert!(full_def.contains("serverConfigType = types.submodule"));
    assert!(full_def.contains("databaseConfigType = types.submodule"));
    assert!(full_def.contains("appConfigType = types.submodule"));

    // appConfigType should reference both nested types
    let app_section_start = full_def
        .find("appConfigType = types.submodule")
        .expect("appConfigType not found");
    let app_section = &full_def[app_section_start..];

    assert!(
        app_section.contains("type = serverConfigType")
            || app_section.contains("type = types.nullOr serverConfigType")
    );
    assert!(
        app_section.contains("type = databaseConfigType")
            || app_section.contains("type = types.nullOr databaseConfigType")
    );
}

#[test]
fn test_full_definition_optional_nested_type() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct BackupConfig {
        #[nixos(description = "Backup path")]
        path: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct AppConfig {
        #[nixos(description = "App name")]
        name: String,

        #[nixos(description = "Optional backup config")]
        backup: Option<BackupConfig>,
    }

    let full_def = AppConfig::nixos_type_full_definition();

    // backupConfigType should be defined
    assert!(full_def.contains("backupConfigType = types.submodule"));

    // Should be referenced with nullOr
    assert!(
        full_def.contains("type = types.nullOr backupConfigType"),
        "Optional nested type should use types.nullOr"
    );
}

#[test]
fn test_full_definition_vec_nested_type() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Item {
        #[nixos(description = "Item name")]
        name: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Container {
        #[nixos(description = "Container name")]
        name: String,

        #[nixos(description = "List of items")]
        items: Vec<Item>,
    }

    let full_def = Container::nixos_type_full_definition();

    // itemType should be defined in let bindings
    assert!(
        full_def.contains("itemType = types.submodule"),
        "Missing itemType definition"
    );

    // Vec field should use types.listOf
    // Note: Currently Vec<CustomType> inlines the type definition rather than
    // referencing the named type. This is a known limitation.
    assert!(
        full_def.contains("type = types.listOf"),
        "Vec field should use types.listOf"
    );
}

#[test]
fn test_full_definition_deeply_nested() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Level3 {
        value: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Level2 {
        data: String,
        nested: Level3,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Level1 {
        name: String,
        child: Level2,
    }

    let full_def = Level1::nixos_type_full_definition();

    // Currently only directly referenced types are collected
    // Level3 is nested within Level2, so it may not be in the top-level let bindings
    // This is a known limitation - only immediate children are collected

    assert!(
        full_def.contains("level2Type = types.submodule"),
        "Level2 should be defined (direct child of Level1)"
    );
    assert!(
        full_def.contains("level1Type = types.submodule"),
        "Level1 should be defined"
    );

    // Level2 should reference Level3 somehow (inline or named)
    let l2_start = full_def.find("level2Type = types.submodule").unwrap();
    let l2_section = &full_def[l2_start..];
    assert!(
        l2_section.contains("nested = lib.mkOption"),
        "Level2 should have nested field"
    );
}

#[test]
fn test_full_definition_indentation() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Inner {
        #[nixos(description = "Inner field")]
        field: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Outer {
        #[nixos(description = "Outer field")]
        inner: Inner,
    }

    let full_def = Outer::nixos_type_full_definition();

    // Check that options are properly indented
    assert!(full_def.contains("    options = {"));

    // Verify consistent indentation for mkOption
    let lines: Vec<&str> = full_def.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("= lib.mkOption {") {
            // Next line should be indented properly
            if i + 1 < lines.len() {
                let next_line = lines[i + 1];
                if next_line.trim().starts_with("type")
                    || next_line.trim().starts_with("description")
                {
                    assert!(
                        next_line.starts_with("        ") || next_line.starts_with("      "),
                        "mkOption properties should be indented"
                    );
                }
            }
        }
    }
}

#[test]
fn test_full_definition_no_duplicates() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct SharedConfig {
        value: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct AppConfig {
        primary: SharedConfig,
        secondary: SharedConfig,
    }

    let full_def = AppConfig::nixos_type_full_definition();

    // sharedConfigType should only be defined once
    let count = full_def
        .matches("sharedConfigType = types.submodule")
        .count();
    assert_eq!(
        count, 1,
        "Shared type should only be defined once, found {} times",
        count
    );

    // But should be referenced twice
    let ref_count = full_def.matches("type = sharedConfigType").count();
    assert_eq!(ref_count, 2, "Shared type should be referenced twice");
}
