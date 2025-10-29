use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_auto_doc_enabled() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct ConfigWithAutoDocs {
        /// This is the first field
        field1: String,

        /// This is the second field
        /// with multiple lines
        field2: u32,

        // Regular comment - should not be picked up
        field3: bool,
    }

    let options = ConfigWithAutoDocs::nixos_options();

    // Doc comments should be used as descriptions
    assert!(options.contains("field1 = lib.mkOption"));
    assert!(options.contains("description = \"This is the first field\""));

    // Multi-line doc comments should be preserved
    assert!(options.contains("field2 = lib.mkOption"));
    assert!(options.contains("description = \"This is the second field\\nwith multiple lines\""));

    // Field without doc comment should not have description
    assert!(options.contains("field3 = lib.mkOption"));
    // Count occurrences - field3 should appear once in the option definition
    let field3_count = options.matches("field3 = lib.mkOption").count();
    assert_eq!(field3_count, 1);
    // But shouldn't have a description line immediately after type
    let field3_section = options.split("field3 = lib.mkOption").nth(1).unwrap();
    let first_lines = field3_section
        .lines()
        .take(3)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!first_lines.contains("description ="));
}

#[test]
fn test_auto_doc_with_explicit_override() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct ConfigWithOverride {
        /// Doc comment description
        #[nixos(description = "Explicit description")]
        field1: String,

        /// Just doc comment
        field2: String,
    }

    let options = ConfigWithOverride::nixos_options();

    // When auto_doc is enabled, doc comments take precedence
    // But explicit #[nixos(description)] should still be available as fallback
    assert!(options.contains("field1 = lib.mkOption"));
    // With auto_doc, doc comment should be preferred
    assert!(options.contains("description = \"Doc comment description\""));

    assert!(options.contains("field2 = lib.mkOption"));
    assert!(options.contains("description = \"Just doc comment\""));
}

#[test]
fn test_auto_doc_disabled_default() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct ConfigWithoutAutoDocs {
        /// Doc comment
        #[nixos(description = "Explicit description")]
        field1: String,

        /// Just doc comment
        field2: String,
    }

    let options = ConfigWithoutAutoDocs::nixos_options();

    // Without auto_doc, explicit description should take precedence
    assert!(options.contains("field1 = lib.mkOption"));
    assert!(options.contains("description = \"Explicit description\""));

    // Without auto_doc, doc comment should still be used as fallback
    assert!(options.contains("field2 = lib.mkOption"));
    assert!(options.contains("description = \"Just doc comment\""));
}

#[test]
fn test_auto_doc_with_all_attributes() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct FullConfig {
        /// The server port
        #[nixos(default = "8080", example = "3000")]
        port: u16,

        /// Enable debug mode
        #[nixos(default = "false")]
        debug: bool,

        /// Internal state field
        #[nixos(internal)]
        internal_field: String,
    }

    let options = FullConfig::nixos_options();

    // All fields should have their doc comments as descriptions
    assert!(options.contains("description = \"The server port\""));
    assert!(options.contains("default = 8080"));
    assert!(options.contains("example = 3000"));

    assert!(options.contains("description = \"Enable debug mode\""));
    assert!(options.contains("default = false"));

    assert!(options.contains("description = \"Internal state field\""));
    assert!(options.contains("internal = true"));
}

#[test]
fn test_auto_doc_nested_structs() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct InnerConfig {
        /// Inner field documentation
        inner_value: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct OuterConfig {
        /// Outer field documentation
        outer_value: String,

        /// Nested configuration
        inner: InnerConfig,
    }

    let outer_options = OuterConfig::nixos_options();
    let inner_options = InnerConfig::nixos_options();

    assert!(outer_options.contains("description = \"Outer field documentation\""));
    assert!(outer_options.contains("description = \"Nested configuration\""));

    assert!(inner_options.contains("description = \"Inner field documentation\""));
}

#[test]
fn test_auto_doc_empty_doc_comment() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct ConfigWithEmptyDocs {
        #[allow(clippy::empty_docs)]
        ///
        empty_doc: String,

        #[allow(clippy::empty_docs)]
        ///
        ///
        whitespace_doc: String,

        /// Valid doc
        valid_doc: String,
    }

    let options = ConfigWithEmptyDocs::nixos_options();

    // Empty doc comments might be picked up but should be empty strings
    assert!(options.contains("empty_doc = lib.mkOption"));

    // Valid doc should work
    assert!(options.contains("description = \"Valid doc\""));
}

#[test]
fn test_auto_doc_with_serde_attributes() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct ConfigWithSerde {
        /// Original name field
        #[serde(rename = "fieldOne")]
        field_one: String,

        /// Skipped field
        #[serde(skip)]
        #[allow(dead_code)]
        skipped: String,

        /// Field with default
        #[serde(default)]
        with_default: Option<String>,
    }

    let options = ConfigWithSerde::nixos_options();

    // Should use serde rename
    assert!(options.contains("fieldOne = lib.mkOption"));
    assert!(options.contains("description = \"Original name field\""));

    // Skipped field should not appear
    assert!(!options.contains("skipped"));

    // Field with serde default should appear
    assert!(options.contains("with_default = lib.mkOption"));
    assert!(options.contains("description = \"Field with default\""));
}

#[test]
fn test_auto_doc_type_definition() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct ServerConfig {
        /// Server port number
        port: u16,
    }

    let type_def = ServerConfig::nixos_type_definition();

    // Should include the doc comment in the generated type definition
    assert!(type_def.contains("serverConfigType = types.submodule"));
    assert!(type_def.contains("description = \"Server port number\""));
}

#[test]
fn test_auto_doc_full_definition() {
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct TestConfig {
        /// Configuration name
        name: String,
    }

    let full_def = TestConfig::nixos_type_full_definition();

    // Should include doc comment in full definition
    assert!(full_def.contains("let"));
    assert!(full_def.contains("testConfigType = types.submodule"));
    assert!(full_def.contains("description = \"Configuration name\""));
}

#[test]
fn test_mixed_auto_doc_configs() {
    // One struct with auto_doc
    #[derive(Serialize, Deserialize, NixosType)]
    #[nixos(auto_doc)]
    struct AutoDocConfig {
        /// Auto doc field
        field1: String,
    }

    // One struct without
    #[derive(Serialize, Deserialize, NixosType)]
    struct ManualDocConfig {
        /// Manual doc field
        #[nixos(description = "Manual description")]
        field1: String,
    }

    let auto_options = AutoDocConfig::nixos_options();
    let manual_options = ManualDocConfig::nixos_options();

    // Auto doc should use doc comment
    assert!(auto_options.contains("description = \"Auto doc field\""));

    // Manual should use explicit description
    assert!(manual_options.contains("description = \"Manual description\""));
}
