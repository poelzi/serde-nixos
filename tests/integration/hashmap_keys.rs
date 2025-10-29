use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;
use std::collections::HashMap;

#[test]
fn test_hashmap_string_keys() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Config {
        #[nixos(description = "String-keyed map")]
        settings: HashMap<String, String>,
    }

    let options = Config::nixos_options();
    assert!(options.contains("settings = lib.mkOption"));
    // HashMap<String, V> should use types.attrsOf
    assert!(options.contains("types.attrsOf"));
}

#[test]
fn test_hashmap_integer_keys() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct PortConfig {
        #[nixos(description = "Port mappings")]
        ports: HashMap<u16, String>,
    }

    let options = PortConfig::nixos_options();
    assert!(options.contains("ports = lib.mkOption"));

    // Integer-keyed maps are tricky in Nix since attribute names must be strings
    // The implementation might use attrsOf or a different strategy
    println!("Integer-keyed HashMap: {}", options);
}

#[test]
fn test_hashmap_bool_keys() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct FlagMap {
        flags: HashMap<bool, String>,
    }

    let options = FlagMap::nixos_options();
    assert!(options.contains("flags = lib.mkOption"));

    // Boolean keys are unusual but valid in Rust
    println!("Boolean-keyed HashMap: {}", options);
}

#[test]
fn test_hashmap_enum_keys() {
    #[derive(Serialize, Deserialize, NixosType, Hash, Eq, PartialEq)]
    enum Environment {
        Development,
        Staging,
        Production,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct EnvConfig {
        #[nixos(description = "Environment-specific settings")]
        environments: HashMap<Environment, String>,
    }

    let options = EnvConfig::nixos_options();
    assert!(options.contains("environments = lib.mkOption"));

    println!("Enum-keyed HashMap: {}", options);
}

#[test]
fn test_nested_hashmap_complex_values() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct ServiceConfig {
        port: u16,
        enabled: bool,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Services {
        #[nixos(description = "Service configurations")]
        configs: HashMap<String, ServiceConfig>,
    }

    let options = Services::nixos_options();
    assert!(options.contains("configs = lib.mkOption"));
    assert!(options.contains("types.attrsOf"));

    println!("HashMap with complex values: {}", options);
}

#[test]
fn test_hashmap_nested_in_hashmap() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct NestedMaps {
        data: HashMap<String, HashMap<String, u32>>,
    }

    let options = NestedMaps::nixos_options();
    assert!(options.contains("data = lib.mkOption"));

    println!("Nested HashMaps: {}", options);
}

#[test]
fn test_optional_hashmap() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct OptionalMap {
        #[nixos(description = "Optional configuration map")]
        settings: Option<HashMap<String, String>>,
    }

    let options = OptionalMap::nixos_options();
    assert!(options.contains("settings = lib.mkOption"));
    assert!(options.contains("types.nullOr"));

    println!("Optional HashMap: {}", options);
}

#[test]
fn test_hashmap_with_default() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct DefaultMap {
        #[nixos(default = "{}", description = "Map with default")]
        labels: HashMap<String, String>,
    }

    let options = DefaultMap::nixos_options();
    assert!(options.contains("labels = lib.mkOption"));
    assert!(options.contains("default = {}"));

    println!("HashMap with default: {}", options);
}
