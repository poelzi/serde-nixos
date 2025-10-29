use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_primitive_types() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct PrimitiveTypes {
        bool_field: bool,
        u8_field: u8,
        u16_field: u16,
        u32_field: u32,
        u64_field: u64,
        i8_field: i8,
        i16_field: i16,
        i32_field: i32,
        i64_field: i64,
        f32_field: f32,
        f64_field: f64,
        string_field: String,
    }

    let options = PrimitiveTypes::nixos_options();

    // Check that the generated options contain the expected type definitions
    assert!(options.contains("bool_field = lib.mkOption"));
    assert!(options.contains("type = types.bool"));
    assert!(options.contains("u32_field = lib.mkOption"));
    assert!(options.contains("type = types.int"));
    assert!(options.contains("f64_field = lib.mkOption"));
    assert!(options.contains("type = types.float"));
    assert!(options.contains("string_field = lib.mkOption"));
    assert!(options.contains("type = types.str"));
}

#[test]
fn test_optional_types() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct OptionalTypes {
        required: String,
        optional: Option<String>,
        optional_number: Option<u32>,
    }

    let options = OptionalTypes::nixos_options();

    assert!(options.contains("required = lib.mkOption"));
    assert!(options.contains("type = types.str"));
    assert!(options.contains("optional = lib.mkOption"));
    assert!(options.contains("types.nullOr types.str"));
    assert!(options.contains("optional_number = lib.mkOption"));
    assert!(options.contains("types.nullOr types.int"));
}

#[test]
fn test_collection_types() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct CollectionTypes {
        list: Vec<String>,
        numbers: Vec<u32>,
        nested_list: Vec<Vec<String>>,
    }

    let options = CollectionTypes::nixos_options();

    assert!(options.contains("list = lib.mkOption"));
    assert!(options.contains("types.listOf types.str"));
    assert!(options.contains("numbers = lib.mkOption"));
    assert!(options.contains("types.listOf types.int"));
    assert!(options.contains("nested_list = lib.mkOption"));
    assert!(options.contains("types.listOf types.listOf types.str"));
}

#[test]
fn test_attributes_support() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct ConfigWithAttributes {
        #[nixos(description = "The server port")]
        port: u16,

        #[nixos(default = "\"localhost\"", description = "Server hostname")]
        host: String,

        #[nixos(example = "100")]
        max_connections: Option<u32>,

        #[nixos(skip)]
        internal_field: String,
    }

    let options = ConfigWithAttributes::nixos_options();

    // Check descriptions
    assert!(options.contains("description = \"The server port\""));
    assert!(options.contains("description = \"Server hostname\""));

    // Check default
    assert!(options.contains("default = \"localhost\""));

    // Check example
    assert!(options.contains("example = 100"));

    // Check that skipped field is not present
    assert!(!options.contains("internal_field"));
}

#[test]
fn test_serde_rename() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct RenamedFields {
        #[serde(rename = "serverPort")]
        #[nixos(description = "Port number")]
        port: u16,

        #[nixos(rename = "hostName")]
        host: String,
    }

    let options = RenamedFields::nixos_options();

    // Should use nixos rename over serde rename
    assert!(options.contains("hostName = lib.mkOption"));

    // Should use serde rename when no nixos rename
    assert!(options.contains("serverPort = lib.mkOption"));
}
