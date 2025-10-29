use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_nested_structs() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct InnerConfig {
        #[nixos(description = "Inner field")]
        inner_field: String,
        inner_number: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct OuterConfig {
        #[nixos(description = "Outer field")]
        outer_field: String,

        #[nixos(description = "Nested configuration")]
        inner: InnerConfig,
    }

    let options = OuterConfig::nixos_options();

    // Check outer fields
    assert!(options.contains("outer_field = lib.mkOption"));
    assert!(options.contains("description = \"Outer field\""));

    // Check that inner is referenced as submodule
    assert!(options.contains("inner = lib.mkOption"));
    assert!(options.contains("types.submodule"));
    assert!(options.contains("description = \"Nested configuration\""));
}

#[test]
fn test_deeply_nested_structs() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Level3 {
        value: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Level2 {
        level3: Level3,
        data: Vec<String>,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Level1 {
        level2: Level2,
        enabled: bool,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct RootConfig {
        level1: Level1,
        name: String,
    }

    let options = RootConfig::nixos_options();
    let type_name = RootConfig::nixos_type();
    let type_def = RootConfig::nixos_type_definition();

    // Should create submodules for nested structs
    assert!(options.contains("level1 = lib.mkOption"));
    // nixos_type() now returns the type name
    assert_eq!(type_name, "rootConfigType");
    // The full definition contains the submodule
    assert!(type_def.contains("types.submodule"));
}

#[test]
fn test_optional_nested_struct() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        host: String,
        port: u16,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct AppConfig {
        name: String,
        database: Option<DatabaseConfig>,
    }

    let options = AppConfig::nixos_options();

    // Should wrap submodule in nullOr
    assert!(options.contains("database = lib.mkOption"));
    assert!(options.contains("types.nullOr types.submodule"));
}

#[test]
fn test_vec_of_structs() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Plugin {
        name: String,
        enabled: bool,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct AppConfig {
        plugins: Vec<Plugin>,
    }

    let options = AppConfig::nixos_options();

    // Should create listOf submodule
    assert!(options.contains("plugins = lib.mkOption"));
    assert!(options.contains("types.listOf types.submodule"));
}
