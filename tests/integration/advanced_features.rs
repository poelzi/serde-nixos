use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[test]
fn test_doc_comment_extraction() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct ConfigWithDocs {
        /// This is a documented field
        documented_field: String,

        #[nixos(description = "Explicit description takes precedence")]
        /// This doc comment should be ignored
        explicit_description: String,

        undocumented_field: u32,
    }

    let options = ConfigWithDocs::nixos_options();

    // Doc comment should be used as description
    assert!(options.contains("documented_field = lib.mkOption"));
    assert!(options.contains("description = \"This is a documented field\""));

    // Explicit description should take precedence
    assert!(options.contains("explicit_description = lib.mkOption"));
    assert!(options.contains("description = \"Explicit description takes precedence\""));
    assert!(!options.contains("This doc comment should be ignored"));

    // Field without docs should not have description
    assert!(options.contains("undocumented_field = lib.mkOption"));
    assert!(!options.contains(
        "undocumented_field = lib.mkOption {\n      type = types.int;\n      description ="
    ));
}

#[test]
fn test_all_mkoption_attributes() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct FullFeatured {
        #[nixos(
            description = "A field with all attributes",
            default = "42",
            default_text = "\"computed at runtime\"",
            example = "100",
            apply = "x: x * 2",
            internal,
            read_only
        )]
        full_field: u32,

        #[nixos(visible = "false")]
        hidden_field: String,

        #[nixos(related_packages = "[ pkgs.foo pkgs.bar ]")]
        package_field: String,
    }

    let options = FullFeatured::nixos_options();

    // Check all attributes are present
    assert!(options.contains("description = \"A field with all attributes\""));
    assert!(options.contains("default = 42"));
    assert!(options.contains("defaultText = \"computed at runtime\""));
    assert!(options.contains("example = 100"));
    assert!(options.contains("apply = x: x * 2"));
    assert!(options.contains("internal = true"));
    assert!(options.contains("readOnly = true"));

    assert!(options.contains("visible = false"));
    assert!(options.contains("relatedPackages = [ pkgs.foo pkgs.bar ]"));
}

#[test]
fn test_named_type_generation() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct MyConfig {
        field: String,
    }

    let type_name = MyConfig::nixos_type_name();
    assert_eq!(type_name, "myConfigType");

    let type_def = MyConfig::nixos_type_definition();
    assert!(type_def.contains("myConfigType = types.submodule"));
}

#[test]
fn test_full_definition_with_let_chain() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Inner {
        value: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Outer {
        inner: Inner,
    }

    let full_def = Outer::nixos_type_full_definition();

    // Should contain let binding
    assert!(full_def.contains("let"), "Missing 'let' keyword");
    assert!(full_def.contains("in outerType"), "Missing 'in outerType'");

    // Should define innerType in the let bindings
    assert!(
        full_def.contains("innerType = types.submodule"),
        "Missing innerType definition in let bindings"
    );

    // Should define outerType
    assert!(
        full_def.contains("outerType = types.submodule"),
        "Missing outerType definition"
    );

    // Should reference innerType in outerType's options
    assert!(
        full_def.contains("type = innerType") || full_def.contains("type = types.nullOr innerType"),
        "outerType should reference innerType"
    );

    // Verify innerType comes before outerType (dependency order)
    let inner_pos = full_def
        .find("innerType = types.submodule")
        .expect("innerType not found");
    let outer_pos = full_def
        .find("outerType = types.submodule")
        .expect("outerType not found");
    assert!(
        inner_pos < outer_pos,
        "innerType should be defined before outerType"
    );
}

#[test]
fn test_multiline_doc_comments() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct MultilineDoc {
        /// This is a multi-line
        /// documentation comment
        /// that spans multiple lines
        field: String,
    }

    let options = MultilineDoc::nixos_options();

    // Should preserve the multiline doc
    assert!(options.contains("description = \"This is a multi-line\\ndocumentation comment\\nthat spans multiple lines\""));
}

#[test]
fn test_camel_case_attribute_names() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct CamelCaseAttrs {
        #[nixos(defaultText = "\"runtime value\"")]
        field1: String,

        #[nixos(readOnly)]
        field2: String,

        #[nixos(relatedPackages = "[ pkgs.test ]")]
        field3: String,
    }

    let options = CamelCaseAttrs::nixos_options();

    // Should support both camelCase and snake_case
    assert!(options.contains("defaultText = \"runtime value\""));
    assert!(options.contains("readOnly = true"));
    assert!(options.contains("relatedPackages = [ pkgs.test ]"));
}

#[test]
fn test_type_name_consistency() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct ServerConfig {
        port: u16,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct DatabaseConfig {
        host: String,
    }

    assert_eq!(ServerConfig::nixos_type_name(), "serverConfigType");
    assert_eq!(DatabaseConfig::nixos_type_name(), "databaseConfigType");

    // Type should match type name
    assert_eq!(ServerConfig::nixos_type(), "serverConfigType");
    assert_eq!(DatabaseConfig::nixos_type(), "databaseConfigType");
}

#[test]
fn test_escaped_descriptions() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct EscapedDesc {
        /// This has "quotes" in it
        field_with_quotes: String,

        #[nixos(description = "This also has \"quotes\"")]
        explicit_quotes: String,
    }

    let options = EscapedDesc::nixos_options();

    // Should properly escape quotes
    assert!(options.contains("description = \"This has \\\"quotes\\\" in it\""));
    assert!(options.contains("description = \"This also has \\\"quotes\\\"\""));
}
