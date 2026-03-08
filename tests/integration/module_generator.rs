//! Tests for NixosModuleGenerator, type_registration! macro,
//! Option<T> default = null behavior, named container references,
//! and the nixos_module! proc-macro.

use serde::{Deserialize, Serialize};
use serde_nixos::generator::NixosModuleGenerator;
use serde_nixos::{type_registration, NixosType};

// ── Test types ──────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, NixosType)]
#[serde(rename_all = "kebab-case")]
#[nixos(auto_doc)]
struct AgentDef {
    /// Agent name.
    name: String,

    /// Description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    /// Suggested models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    suggested_models: Option<Vec<String>>,

    /// Skills.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    skills: Option<Vec<String>>,

    /// Temperature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Serialize, Deserialize, NixosType)]
#[serde(rename_all = "kebab-case")]
#[nixos(auto_doc)]
struct ArtifactDecl {
    /// Artifact name.
    name: String,
    /// Container path.
    container_path: String,
    /// Description.
    #[serde(default)]
    description: Option<String>,
}

#[derive(Serialize, Deserialize, NixosType)]
#[serde(rename_all = "kebab-case")]
#[nixos(auto_doc)]
struct BranchCond {
    /// Script body.
    script: String,
    /// Description.
    #[serde(default)]
    description: Option<String>,
}

#[derive(Serialize, Deserialize, NixosType)]
#[serde(rename_all = "kebab-case")]
#[nixos(auto_doc)]
struct EdgeDef {
    /// Source step.
    from: String,
    /// Target step.
    to: String,
    /// Optional condition.
    #[serde(default)]
    condition: Option<BranchCond>,
}

#[derive(Serialize, Deserialize, NixosType)]
#[serde(rename_all = "kebab-case")]
#[nixos(auto_doc)]
struct StepDef {
    /// Step name.
    name: String,
    /// Store path to OCI bundle.
    derivation: String,
    /// Timeout.
    #[serde(default)]
    timeout_secs: Option<u64>,
    /// Agent name.
    #[serde(default)]
    agent: Option<String>,
    /// Artifacts.
    #[serde(default)]
    artifacts: Option<Vec<ArtifactDecl>>,
}

#[derive(Serialize, Deserialize, NixosType)]
#[serde(rename_all = "kebab-case")]
#[nixos(auto_doc)]
struct WorkflowDef {
    /// Workflow name.
    name: String,
    /// Description.
    #[serde(default)]
    description: Option<String>,
    /// Steps.
    steps: Vec<StepDef>,
    /// Edges.
    edges: Vec<EdgeDef>,
    /// Agents.
    #[serde(default)]
    agents: Option<Vec<AgentDef>>,
    /// Default timeout.
    #[serde(default)]
    default_timeout_secs: Option<u64>,
}

// ── A1: Option<T> default = null ────────────────────────────────────

#[test]
fn test_option_string_gets_default_null() {
    let options = AgentDef::nixos_options();
    // description is Option<String> -> should have "default = null;"
    assert!(
        options.contains("default = null;"),
        "Option<String> fields should emit 'default = null;'\nGot:\n{}",
        options
    );
}

#[test]
fn test_option_vec_gets_default_null() {
    let options = AgentDef::nixos_options();
    // suggested-models is Option<Vec<String>>
    // Find the section for suggested-models and check it has default = null
    let sections: Vec<&str> = options.split("lib.mkOption {").collect();
    let models_section = sections
        .iter()
        .find(|s| s.contains("Suggested models"))
        .expect("Should have suggested-models section");
    assert!(
        models_section.contains("default = null;"),
        "Option<Vec<String>> should emit 'default = null;'\nSection:\n{}",
        models_section
    );
}

#[test]
fn test_option_float_gets_default_null() {
    let options = AgentDef::nixos_options();
    let sections: Vec<&str> = options.split("lib.mkOption {").collect();
    let temp_section = sections
        .iter()
        .find(|s| s.contains("Temperature"))
        .expect("Should have temperature section");
    assert!(
        temp_section.contains("default = null;"),
        "Option<f64> should emit 'default = null;'\nSection:\n{}",
        temp_section
    );
}

#[test]
fn test_required_field_no_default() {
    let options = AgentDef::nixos_options();
    let sections: Vec<&str> = options.split("lib.mkOption {").collect();
    let name_section = sections
        .iter()
        .find(|s| s.contains("Agent name"))
        .expect("Should have name section");
    assert!(
        !name_section.contains("default ="),
        "Required String field should NOT have a default\nSection:\n{}",
        name_section
    );
}

#[test]
fn test_explicit_nixos_default_overrides_option_null() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct WithExplicitDefault {
        #[nixos(default = "[ ]")]
        items: Option<Vec<String>>,
    }

    let options = WithExplicitDefault::nixos_options();
    assert!(
        options.contains("default = [ ];"),
        "Explicit #[nixos(default)] should override auto null\nGot:\n{}",
        options
    );
    // Should NOT also contain "default = null;"
    assert!(
        !options.contains("default = null;"),
        "Should not have BOTH explicit default and null\nGot:\n{}",
        options
    );
}

#[test]
fn test_option_custom_type_gets_default_null() {
    let options = EdgeDef::nixos_options();
    // condition is Option<BranchCond>
    assert!(
        options.contains("default = null;"),
        "Option<CustomType> should emit 'default = null;'\nGot:\n{}",
        options
    );
}

// ── A2: Named container references in full_definition ───────────────

#[test]
fn test_vec_custom_type_uses_named_reference() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Item {
        name: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Container {
        items: Vec<Item>,
    }

    let full = Container::nixos_type_full_definition();
    // Should reference itemType by name, not inline submodule
    assert!(
        full.contains("types.listOf itemType"),
        "Vec<CustomType> should use named reference 'types.listOf itemType'\nGot:\n{}",
        full
    );
    // itemType should be defined in let block
    assert!(
        full.contains("itemType = types.submodule"),
        "itemType should be defined in let block\nGot:\n{}",
        full
    );
}

#[test]
fn test_option_vec_custom_type_uses_named_reference() {
    #[derive(Serialize, Deserialize, NixosType)]
    struct Tag {
        value: String,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Parent {
        tags: Option<Vec<Tag>>,
    }

    let full = Parent::nixos_type_full_definition();
    // Option<Vec<Tag>> -> types.nullOr (types.listOf tagType)
    assert!(
        full.contains("types.nullOr (types.listOf tagType)"),
        "Option<Vec<Custom>> should be 'types.nullOr (types.listOf tagType)'\nGot:\n{}",
        full
    );
}

#[test]
fn test_hashmap_custom_type_uses_named_reference() {
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, NixosType)]
    struct Entry {
        value: u32,
    }

    #[derive(Serialize, Deserialize, NixosType)]
    struct Registry {
        entries: HashMap<String, Entry>,
    }

    let full = Registry::nixos_type_full_definition();
    assert!(
        full.contains("types.attrsOf entryType"),
        "HashMap<String, Custom> should use named reference 'types.attrsOf entryType'\nGot:\n{}",
        full
    );
}

// ── B: type_registration! macro and NixosModuleGenerator ────────────

#[test]
fn test_type_registration_macro() {
    let reg = type_registration!(AgentDef);
    assert_eq!(reg.type_name, "agentDefType");
    assert!(!reg.options.is_empty());
    assert_eq!(reg.type_expr, "agentDefType");
}

#[test]
fn test_module_generator_realistic() {
    let nix = NixosModuleGenerator::new()
        .header("Generated workflow type definitions.\nDo not edit manually.")
        .register(type_registration!(ArtifactDecl))
        .register(type_registration!(BranchCond))
        .register(type_registration!(AgentDef))
        .register(type_registration!(EdgeDef))
        .register(type_registration!(StepDef))
        .register(type_registration!(WorkflowDef))
        .export_type("agentDefType")
        .export_type("stepDefType")
        .export_type("edgeDefType")
        .export_type("workflowDefType")
        .generate();

    // Header
    assert!(nix.contains("# Generated workflow type definitions."));
    assert!(nix.contains("# Do not edit manually."));

    // Module structure
    assert!(nix.contains("{ lib, ... }:"));
    assert!(nix.contains("with lib;"));
    assert!(nix.contains("let"));
    assert!(nix.contains("in"));

    // All types defined
    assert!(nix.contains("artifactDeclType = types.submodule {"));
    assert!(nix.contains("branchCondType = types.submodule {"));
    assert!(nix.contains("agentDefType = types.submodule {"));
    assert!(nix.contains("edgeDefType = types.submodule {"));
    assert!(nix.contains("stepDefType = types.submodule {"));
    assert!(nix.contains("workflowDefType = types.submodule {"));

    // Options content (spot checks)
    assert!(nix.contains("type = types.str;"));
    assert!(nix.contains("type = types.nullOr types.str;"));
    assert!(nix.contains("default = null;"));

    // Exports
    assert!(nix.contains("inherit"));
    assert!(nix.contains("agentDefType"));
    assert!(nix.contains("stepDefType"));
    assert!(nix.contains("workflowDefType"));

    // Ordering: leaf types before composite types
    let artifact_pos = nix.find("artifactDeclType = types.submodule").unwrap();
    let step_pos = nix.find("stepDefType = types.submodule").unwrap();
    let workflow_pos = nix.find("workflowDefType = types.submodule").unwrap();
    assert!(artifact_pos < step_pos);
    assert!(step_pos < workflow_pos);
}

#[test]
fn test_module_generator_custom_args() {
    let nix = NixosModuleGenerator::new()
        .args("{ config, lib, pkgs, ... }:")
        .clear_preamble()
        .preamble("with lib;")
        .preamble("with builtins;")
        .register(type_registration!(BranchCond))
        .export_all_types()
        .generate();

    assert!(nix.contains("{ config, lib, pkgs, ... }:"));
    assert!(nix.contains("with lib;"));
    assert!(nix.contains("with builtins;"));
}

#[test]
fn test_module_generator_let_binding() {
    let nix = NixosModuleGenerator::new()
        .register(type_registration!(AgentDef))
        .let_binding("cfg = config.services.hive;")
        .export_all_types()
        .generate();

    assert!(nix.contains("cfg = config.services.hive;"));
}

#[test]
fn test_module_generator_custom_export() {
    let agent_options = AgentDef::nixos_options();
    let nix = NixosModuleGenerator::new()
        .register(type_registration!(AgentDef))
        .export_type("agentDefType")
        .export_custom("agentOptions", &agent_options)
        .generate();

    assert!(nix.contains("agentOptions = {"));
}

// ── C: nixos_module! proc-macro ─────────────────────────────────────

#[test]
fn test_nixos_module_macro() {
    // nixos_module! should produce nixos_type_full_definition() output
    let from_macro: String = serde_nixos::nixos_module!(EdgeDef);
    let from_method = EdgeDef::nixos_type_full_definition();
    assert_eq!(from_macro, from_method);
}

#[test]
fn test_nixos_module_macro_has_let_bindings() {
    let nix: String = serde_nixos::nixos_module!(EdgeDef);
    // EdgeDef contains Option<BranchCond>, so BranchCond should be in let block
    assert!(nix.contains("let"));
    assert!(nix.contains("branchCondType = types.submodule"));
    assert!(nix.contains("in edgeDefType"));
}

// ── Kebab-case integration ──────────────────────────────────────────

#[test]
fn test_kebab_case_field_names_in_generator() {
    let nix = NixosModuleGenerator::new()
        .register(type_registration!(AgentDef))
        .export_all_types()
        .generate();

    // snake_case fields should be kebab-case in output
    assert!(nix.contains("suggested-models = lib.mkOption"));
    assert!(!nix.contains("suggested_models = lib.mkOption"));
}

#[test]
fn test_kebab_case_with_default_null() {
    // Directly check the options output (not via generator wrapper)
    let options = StepDef::nixos_options();

    // timeout-secs is Option<u64>, should have "default = null;"
    assert!(
        options.contains("timeout-secs"),
        "Should have kebab-case field 'timeout-secs'\nGot:\n{}",
        options
    );

    // Find the timeout-secs mkOption block and verify it has default = null
    let timeout_pos = options.find("timeout-secs").unwrap();
    let after_timeout = &options[timeout_pos..];
    // The next "}" closes the mkOption block
    let block_end = after_timeout.find("};").unwrap();
    let timeout_block = &after_timeout[..block_end];

    assert!(
        timeout_block.contains("default = null;"),
        "timeout-secs (Option<u64>) should have default = null\nBlock:\n{}",
        timeout_block
    );
}
