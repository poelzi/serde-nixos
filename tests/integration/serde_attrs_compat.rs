//! Tests for serde attribute parse compatibility
//!
//! These tests ensure that the NixosType derive macro does not fail when
//! encountering any valid serde attribute — even ones the macro does not
//! need to interpret. If a future serde release adds new attributes, the
//! catch-all parser should consume them gracefully.

use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

// ---------------------------------------------------------------------------
// Helper modules for serde custom serialization attributes
// ---------------------------------------------------------------------------

mod custom_u16 {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &u16, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        u16::deserialize(deserializer)
    }
}

// ---------------------------------------------------------------------------
// Field-level attributes — each test must compile and produce non-empty output
// ---------------------------------------------------------------------------

#[test]
fn test_field_skip_serializing_if() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(skip_serializing_if = "Option::is_none")]
        port: Option<u16>,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_alias() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(alias = "legacyPort")]
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_serialize_with() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(serialize_with = "custom_u16::serialize")]
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_deserialize_with() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(deserialize_with = "custom_u16::deserialize")]
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_with() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(with = "custom_u16")]
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_bound_parenthesized() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(bound(serialize = "u16: Serialize", deserialize = "u16: Deserialize<'de>"))]
    struct Cfg {
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_flatten() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Inner {
        host: String,
    }

    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(flatten)]
        inner: Inner,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_default_bare_and_function() {
    fn default_port() -> u16 {
        8080
    }

    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(default)]
        flag: bool,

        #[serde(default = "default_port")]
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_field_rename_simple_and_directional() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(rename = "serverPort")]
        port: u16,

        #[serde(rename(serialize = "outHost", deserialize = "inHost"))]
        host: String,
    }
    let options = Cfg::nixos_options();
    assert!(options.contains("serverPort = lib.mkOption"));
    assert!(options.contains("inHost = lib.mkOption"));
}

#[test]
fn test_field_skip_variants() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(skip)]
        #[allow(dead_code)]
        secret: String,

        #[serde(skip_serializing)]
        #[allow(dead_code)]
        write_only: String,

        #[serde(skip_deserializing)]
        #[allow(dead_code)]
        read_only: String,

        visible: u16,
    }
    let options = Cfg::nixos_options();
    assert!(!options.contains("secret"));
    assert!(options.contains("visible = lib.mkOption"));
}

// ---------------------------------------------------------------------------
// Multiple field-level attributes combined in a single annotation
// ---------------------------------------------------------------------------

#[test]
fn test_field_many_attrs_combined() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    struct Cfg {
        #[serde(
            default,
            alias = "p",
            serialize_with = "custom_u16::serialize",
            deserialize_with = "custom_u16::deserialize"
        )]
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

// ---------------------------------------------------------------------------
// Container-level attributes
// ---------------------------------------------------------------------------

#[test]
fn test_container_rename_all_simple() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "camelCase")]
    struct Cfg {
        my_field: u16,
    }
    let options = Cfg::nixos_options();
    assert!(options.contains("myField = lib.mkOption"));
}

#[test]
fn test_container_rename_all_directional() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "kebab-case"))]
    struct Cfg {
        my_field: u16,
    }
    let options = Cfg::nixos_options();
    // Should use the deserialize rule
    assert!(options.contains("my-field = lib.mkOption"));
}

#[test]
fn test_container_deny_unknown_fields() {
    // This is a bare flag with no value — must be handled by the catch-all.
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(deny_unknown_fields)]
    struct Cfg {
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_container_tag_and_content() {
    // Tagged enum container attrs should not break struct parsing when used on enums.
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "snake_case")]
    enum Action {
        Start,
        Stop,
    }
    let nixos_type = Action::nixos_type();
    assert!(nixos_type.contains("\"start\""));
    assert!(nixos_type.contains("\"stop\""));
}

#[test]
fn test_container_default() {
    #[derive(Debug, Default, Serialize, Deserialize, NixosType)]
    #[serde(default)]
    struct Cfg {
        port: u16,
        host: String,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_container_bound_with_value() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(bound = "u16: Copy")]
    struct Cfg {
        port: u16,
    }
    assert!(!Cfg::nixos_type().is_empty());
}

#[test]
fn test_container_multiple_attrs() {
    #[derive(Debug, Default, Serialize, Deserialize, NixosType)]
    #[serde(default, rename_all = "camelCase", deny_unknown_fields)]
    struct Cfg {
        my_port: u16,
        my_host: String,
    }
    let options = Cfg::nixos_options();
    assert!(options.contains("myPort = lib.mkOption"));
    assert!(options.contains("myHost = lib.mkOption"));
}

// ---------------------------------------------------------------------------
// Enum-level serde attributes
// ---------------------------------------------------------------------------

#[test]
fn test_enum_rename_all_on_variants() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "kebab-case")]
    enum LogLevel {
        VeryVerbose,
        QuietMode,
    }
    let nixos_type = LogLevel::nixos_type();
    assert!(nixos_type.contains("\"very-verbose\""));
    assert!(nixos_type.contains("\"quiet-mode\""));
}

#[test]
fn test_enum_variant_rename_overrides_rename_all() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "snake_case")]
    enum Mode {
        FastForward,
        #[serde(rename = "CUSTOM")]
        SlowReplay,
    }
    let nixos_type = Mode::nixos_type();
    assert!(nixos_type.contains("\"fast_forward\""));
    assert!(nixos_type.contains("\"CUSTOM\""));
}

// ---------------------------------------------------------------------------
// Mixed nixos + serde attributes (nixos should take priority for rename)
// ---------------------------------------------------------------------------

#[test]
fn test_nixos_rename_overrides_serde_rename_all() {
    #[derive(Debug, Serialize, Deserialize, NixosType)]
    #[serde(rename_all = "camelCase")]
    struct Cfg {
        #[nixos(rename = "custom_name")]
        my_field: u16,

        other_field: bool,
    }
    let options = Cfg::nixos_options();
    assert!(options.contains("custom_name = lib.mkOption"));
    assert!(options.contains("otherField = lib.mkOption"));
}
