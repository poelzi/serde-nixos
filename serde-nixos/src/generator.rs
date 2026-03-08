//! Generator utilities for creating NixOS modules from Rust types
//!
//! This module provides two builder APIs:
//!
//! - [`NixosModuleBuilder`] — generates a complete NixOS service module
//!   (`{ config, lib, pkgs, ... }:`) with options, imports, and config.
//!
//! - [`NixosModuleGenerator`] — generates a `.nix` file containing
//!   `types.submodule` definitions with `let` bindings and typed exports.
//!   Designed for use with the [`type_registration!`] macro to compose
//!   multiple `#[derive(NixosType)]` structs into a single, self-consistent
//!   type definitions file.

use std::fmt::Write;

// ── NixosModuleBuilder (existing API) ───────────────────────────────

/// Builder for generating NixOS module definitions
pub struct NixosModuleBuilder {
    module_name: String,
    options: Vec<NixosOption>,
    imports: Vec<String>,
    config_lines: Vec<String>,
}

impl NixosModuleBuilder {
    /// Create a new module builder
    pub fn new(module_name: impl Into<String>) -> Self {
        Self {
            module_name: module_name.into(),
            options: Vec::new(),
            imports: Vec::new(),
            config_lines: Vec::new(),
        }
    }

    /// Add an option to the module
    pub fn add_option(&mut self, option: NixosOption) -> &mut Self {
        self.options.push(option);
        self
    }

    /// Add an import to the module
    pub fn add_import(&mut self, import: impl Into<String>) -> &mut Self {
        self.imports.push(import.into());
        self
    }

    /// Add a config line to the module
    pub fn add_config_line(&mut self, line: impl Into<String>) -> &mut Self {
        self.config_lines.push(line.into());
        self
    }

    /// Build the complete NixOS module
    pub fn build(&self) -> String {
        let mut result = String::new();

        // Header
        writeln!(result, "{{ config, lib, pkgs, ... }}:").unwrap();
        writeln!(result).unwrap();
        writeln!(result, "with lib;").unwrap();
        writeln!(result).unwrap();
        writeln!(result, "{{").unwrap();

        // Imports
        if !self.imports.is_empty() {
            writeln!(result, "  imports = [").unwrap();
            for import in &self.imports {
                writeln!(result, "    {}", import).unwrap();
            }
            writeln!(result, "  ];").unwrap();
            writeln!(result).unwrap();
        }

        // Options
        writeln!(result, "  options.{} = {{", self.module_name).unwrap();
        for option in &self.options {
            write!(result, "{}", option.to_nix(2)).unwrap();
        }
        writeln!(result, "  }};").unwrap();

        // Config
        if !self.config_lines.is_empty() {
            writeln!(result).unwrap();
            writeln!(
                result,
                "  config = mkIf config.{}.enable {{",
                self.module_name
            )
            .unwrap();
            for line in &self.config_lines {
                writeln!(result, "    {}", line).unwrap();
            }
            writeln!(result, "  }};").unwrap();
        }

        writeln!(result, "}}").unwrap();
        result
    }
}

/// Represents a NixOS option
pub struct NixosOption {
    pub name: String,
    pub type_expr: String,
    pub description: Option<String>,
    pub default: Option<String>,
    pub example: Option<String>,
}

impl NixosOption {
    /// Create a new option
    pub fn new(name: impl Into<String>, type_expr: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_expr: type_expr.into(),
            description: None,
            default: None,
            example: None,
        }
    }

    /// Set the description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set the default value
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Set an example value
    pub fn example(mut self, example: impl Into<String>) -> Self {
        self.example = Some(example.into());
        self
    }

    /// Convert to Nix syntax
    pub fn to_nix(&self, indent: usize) -> String {
        let mut result = String::new();
        let spaces = " ".repeat(indent);

        writeln!(result, "{}  {} = lib.mkOption {{", spaces, self.name).unwrap();
        writeln!(result, "{}    type = {};", spaces, self.type_expr).unwrap();

        if let Some(desc) = &self.description {
            writeln!(result, "{}    description = \"{}\";", spaces, desc).unwrap();
        }

        if let Some(default) = &self.default {
            writeln!(result, "{}    default = {};", spaces, default).unwrap();
        }

        if let Some(example) = &self.example {
            writeln!(result, "{}    example = {};", spaces, example).unwrap();
        }

        writeln!(result, "{}  }};", spaces).unwrap();
        result
    }
}

// ── NixosModuleGenerator (new API) ──────────────────────────────────

/// Registration of a Rust type for inclusion in a generated NixOS module.
///
/// Captures the output of the `#[derive(NixosType)]` inherent methods so
/// that [`NixosModuleGenerator`] can compose multiple types into a single
/// `.nix` file with proper `let` bindings and cross-references.
///
/// Use the [`type_registration!`](crate::type_registration) macro to
/// create instances from any `#[derive(NixosType)]` struct:
///
/// ```ignore
/// use serde_nixos::type_registration;
///
/// let reg = type_registration!(AgentDefinition);
/// ```
#[derive(Debug, Clone)]
pub struct TypeRegistration {
    /// The NixOS type name (e.g. `"agentDefinitionType"`).
    ///
    /// This is the camelCase name used as a `let` binding and for
    /// cross-references from other types' option definitions.
    pub type_name: &'static str,

    /// The options body generated by `T::nixos_options()`.
    ///
    /// Contains the `lib.mkOption { ... };` entries for each field,
    /// **without** a wrapping `types.submodule { options = { ... }; };`.
    pub options: String,

    /// The type expression generated by `T::nixos_type()`.
    ///
    /// For named structs this is typically the type name itself
    /// (e.g. `"agentDefinitionType"`).
    pub type_expr: String,
}

/// What to export from the generated `in { ... }` block.
#[derive(Debug, Clone)]
pub enum Export {
    /// Inherit a type name from the `let` block.
    InheritType(String),
    /// A custom named attribute with a raw Nix expression body.
    Custom {
        /// Attribute name in the exported set.
        name: String,
        /// Raw Nix expression (will be indented but not otherwise processed).
        body: String,
    },
}

/// Builder for generating a complete `.nix` file containing NixOS
/// `types.submodule` definitions with `let` bindings.
///
/// Unlike [`NixosModuleBuilder`] (which targets full NixOS service modules),
/// this generator focuses on **type definition files** — the kind of `.nix`
/// file that other builders import to get typed submodule definitions.
///
/// # Example
///
/// ```ignore
/// use serde_nixos::{type_registration, generator::NixosModuleGenerator};
///
/// let nix = NixosModuleGenerator::new()
///     .header("Auto-generated workflow type definitions.\n# Do not edit.")
///     .register(type_registration!(BranchCondition))
///     .register(type_registration!(EdgeDefinition))
///     .register(type_registration!(AgentDefinition))
///     .register(type_registration!(StepDefinition))
///     .register(type_registration!(WorkflowDefinition))
///     .export_type("agentDefinitionType")
///     .export_type("stepDefinitionType")
///     .export_type("workflowDefinitionType")
///     .generate();
///
/// std::fs::write("nix/lib/workflow-types.nix", nix).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct NixosModuleGenerator {
    types: Vec<TypeRegistration>,
    header: Option<String>,
    args: String,
    preamble: Vec<String>,
    extra_let_bindings: Vec<String>,
    exports: Vec<Export>,
    indent_width: usize,
}

impl Default for NixosModuleGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NixosModuleGenerator {
    /// Create a new generator with sensible defaults.
    ///
    /// Default function arguments: `{ lib, ... }:`
    /// Default preamble: `with lib;`
    /// Default indent width: 2 spaces
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            header: None,
            args: "{ lib, ... }:".to_string(),
            preamble: vec!["with lib;".to_string()],
            extra_let_bindings: Vec::new(),
            exports: Vec::new(),
            indent_width: 2,
        }
    }

    /// Set the file header comment.
    ///
    /// Each line is prefixed with `# `. Multi-line strings are split on
    /// newlines and each line gets the prefix. Lines that already start
    /// with `#` are left as-is.
    pub fn header(mut self, comment: &str) -> Self {
        self.header = Some(comment.to_string());
        self
    }

    /// Override the module function arguments (default `"{ lib, ... }:"`).
    pub fn args(mut self, args: &str) -> Self {
        self.args = args.to_string();
        self
    }

    /// Add a preamble line (e.g. `"with lib;"`).
    ///
    /// The default preamble already contains `"with lib;"`. Call this to
    /// add additional lines. Use [`clear_preamble`](Self::clear_preamble)
    /// first if you want to replace the default.
    pub fn preamble(mut self, line: &str) -> Self {
        self.preamble.push(line.to_string());
        self
    }

    /// Clear all preamble lines (including the default `"with lib;"`).
    pub fn clear_preamble(mut self) -> Self {
        self.preamble.clear();
        self
    }

    /// Register a type for inclusion in the generated `let` block.
    ///
    /// Types are emitted in registration order. Register leaf types
    /// (those without dependencies on other custom types) first, then
    /// composite types that reference them.
    ///
    /// ```ignore
    /// generator
    ///     .register(type_registration!(BranchCondition))  // leaf
    ///     .register(type_registration!(EdgeDefinition))    // uses BranchCondition
    ///     .register(type_registration!(StepDefinition))    // uses ArtifactDeclaration
    ///     .register(type_registration!(WorkflowDefinition)) // uses all above
    /// ```
    pub fn register(mut self, reg: TypeRegistration) -> Self {
        self.types.push(reg);
        self
    }

    /// Add a raw `let` binding (arbitrary Nix expression).
    ///
    /// The binding is emitted after all type definitions in the `let` block.
    pub fn let_binding(mut self, nix: &str) -> Self {
        self.extra_let_bindings.push(nix.to_string());
        self
    }

    /// Export (inherit) a registered type by its NixOS type name.
    ///
    /// The type name must match one of the registered types' `type_name`.
    pub fn export_type(mut self, type_name: &str) -> Self {
        self.exports
            .push(Export::InheritType(type_name.to_string()));
        self
    }

    /// Export all registered types via `inherit`.
    pub fn export_all_types(mut self) -> Self {
        for reg in &self.types {
            self.exports
                .push(Export::InheritType(reg.type_name.to_string()));
        }
        self
    }

    /// Export a custom attribute with a raw Nix expression body.
    ///
    /// ```ignore
    /// generator.export_custom("agentOptions", &AgentDefinition::nixos_options())
    /// ```
    pub fn export_custom(mut self, name: &str, body: &str) -> Self {
        self.exports.push(Export::Custom {
            name: name.to_string(),
            body: body.to_string(),
        });
        self
    }

    /// Set the indentation width in spaces (default: 2).
    pub fn indent_width(mut self, width: usize) -> Self {
        self.indent_width = width;
        self
    }

    /// Generate the complete `.nix` file as a string.
    pub fn generate(&self) -> String {
        let w = self.indent_width;
        let i1 = " ".repeat(w); // 1 level
        let i2 = " ".repeat(w * 2); // 2 levels
        let i3 = " ".repeat(w * 3); // 3 levels

        let mut out = String::new();

        // ── Header comment ──────────────────────────────────────
        if let Some(header) = &self.header {
            for line in header.lines() {
                if line.is_empty() {
                    writeln!(out, "#").unwrap();
                } else if line.starts_with('#') {
                    writeln!(out, "{}", line).unwrap();
                } else {
                    writeln!(out, "# {}", line).unwrap();
                }
            }
            writeln!(out).unwrap();
        }

        // ── Function arguments ──────────────────────────────────
        writeln!(out, "{}", self.args).unwrap();
        writeln!(out).unwrap();

        // ── Preamble ────────────────────────────────────────────
        for line in &self.preamble {
            writeln!(out, "{}", line).unwrap();
        }
        if !self.preamble.is_empty() {
            writeln!(out).unwrap();
        }

        // ── Let block ───────────────────────────────────────────
        if !self.types.is_empty() || !self.extra_let_bindings.is_empty() {
            writeln!(out, "let").unwrap();

            // Type definitions
            for reg in &self.types {
                if reg.type_expr.trim() == reg.type_name {
                    // Struct-like registrations reference themselves as a named
                    // type and need the submodule body from `options`.
                    writeln!(out, "{}{} = types.submodule {{", i1, reg.type_name).unwrap();
                    writeln!(out, "{}options = {{", i2).unwrap();

                    // Indent each options line by 3 levels
                    for line in reg.options.lines() {
                        if line.trim().is_empty() {
                            writeln!(out).unwrap();
                        } else {
                            writeln!(out, "{}{}", i3, line.trim_start()).unwrap();
                        }
                    }

                    writeln!(out, "{}}};", i2).unwrap();
                    writeln!(out, "{}}};", i1).unwrap();
                    writeln!(out).unwrap();
                } else {
                    // Enum-like registrations already have a complete type
                    // expression (e.g. `types.enum [ ... ]`).
                    writeln!(out, "{}{} = {};", i1, reg.type_name, reg.type_expr.trim()).unwrap();
                    writeln!(out).unwrap();
                }
            }

            // Extra let bindings
            for binding in &self.extra_let_bindings {
                for line in binding.lines() {
                    if line.trim().is_empty() {
                        writeln!(out).unwrap();
                    } else {
                        writeln!(out, "{}{}", i1, line).unwrap();
                    }
                }
                writeln!(out).unwrap();
            }

            writeln!(out, "in").unwrap();
        }

        // ── Exports ─────────────────────────────────────────────
        writeln!(out, "{{").unwrap();

        // Collect inherit types
        let inherit_types: Vec<&str> = self
            .exports
            .iter()
            .filter_map(|e| match e {
                Export::InheritType(name) => Some(name.as_str()),
                _ => None,
            })
            .collect();

        if !inherit_types.is_empty() {
            write!(out, "{}inherit", i1).unwrap();
            for name in &inherit_types {
                write!(out, "\n{}{}", i2, name).unwrap();
            }
            writeln!(out, "\n{};", i2).unwrap();
        }

        // Custom exports
        for export in &self.exports {
            if let Export::Custom { name, body } = export {
                writeln!(out).unwrap();
                writeln!(out, "{}{} = {{", i1, name).unwrap();
                for line in body.lines() {
                    if line.trim().is_empty() {
                        writeln!(out).unwrap();
                    } else {
                        writeln!(out, "{}{}", i2, line.trim_start()).unwrap();
                    }
                }
                writeln!(out, "{}}};", i1).unwrap();
            }
        }

        writeln!(out, "}}").unwrap();

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_generator_empty() {
        let gen = NixosModuleGenerator::new();
        let output = gen.generate();
        assert!(output.contains("{ lib, ... }:"));
        assert!(output.contains("with lib;"));
        assert!(output.contains("{"));
        assert!(output.contains("}"));
    }

    #[test]
    fn test_module_generator_with_header() {
        let gen = NixosModuleGenerator::new().header("Auto-generated.\nDo not edit.");
        let output = gen.generate();
        assert!(output.contains("# Auto-generated."));
        assert!(output.contains("# Do not edit."));
    }

    #[test]
    fn test_module_generator_with_type() {
        let reg = TypeRegistration {
            type_name: "fooType",
            options:
                "name = lib.mkOption {\n  type = types.str;\n  description = \"The name\";\n};\n"
                    .to_string(),
            type_expr: "fooType".to_string(),
        };
        let gen = NixosModuleGenerator::new()
            .register(reg)
            .export_type("fooType");
        let output = gen.generate();

        assert!(output.contains("let"));
        assert!(output.contains("fooType = types.submodule {"));
        assert!(output.contains("options = {"));
        assert!(output.contains("name = lib.mkOption {"));
        assert!(output.contains("type = types.str;"));
        assert!(output.contains("in"));
        assert!(output.contains("inherit"));
        assert!(output.contains("fooType"));
    }

    #[test]
    fn test_module_generator_multiple_types() {
        let leaf = TypeRegistration {
            type_name: "leafType",
            options: "value = lib.mkOption {\n  type = types.str;\n};\n".to_string(),
            type_expr: "leafType".to_string(),
        };
        let parent = TypeRegistration {
            type_name: "parentType",
            options: "child = lib.mkOption {\n  type = leafType;\n};\n".to_string(),
            type_expr: "parentType".to_string(),
        };
        let gen = NixosModuleGenerator::new()
            .register(leaf)
            .register(parent)
            .export_type("leafType")
            .export_type("parentType");
        let output = gen.generate();

        // Verify ordering: leafType before parentType
        let leaf_pos = output.find("leafType = types.submodule").unwrap();
        let parent_pos = output.find("parentType = types.submodule").unwrap();
        assert!(leaf_pos < parent_pos);
    }

    #[test]
    fn test_module_generator_custom_export() {
        let reg = TypeRegistration {
            type_name: "myType",
            options: "x = lib.mkOption {\n  type = types.int;\n};\n".to_string(),
            type_expr: "myType".to_string(),
        };
        let gen = NixosModuleGenerator::new()
            .register(reg.clone())
            .export_type("myType")
            .export_custom("myOptions", &reg.options);
        let output = gen.generate();

        assert!(output.contains("myOptions = {"));
        assert!(output.contains("x = lib.mkOption {"));
    }

    #[test]
    fn test_module_generator_export_all() {
        let a = TypeRegistration {
            type_name: "alphaType",
            options: String::new(),
            type_expr: "alphaType".to_string(),
        };
        let b = TypeRegistration {
            type_name: "betaType",
            options: String::new(),
            type_expr: "betaType".to_string(),
        };
        let gen = NixosModuleGenerator::new()
            .register(a)
            .register(b)
            .export_all_types();
        let output = gen.generate();

        assert!(output.contains("alphaType"));
        assert!(output.contains("betaType"));
        assert!(output.contains("inherit"));
    }

    #[test]
    fn test_module_generator_uses_type_expr_for_non_submodule_types() {
        let enum_like = TypeRegistration {
            type_name: "modeType",
            options: String::new(),
            type_expr: "types.enum [ \"fast\" \"safe\" ]".to_string(),
        };

        let gen = NixosModuleGenerator::new()
            .register(enum_like)
            .export_type("modeType");
        let output = gen.generate();

        assert!(output.contains("modeType = types.enum [ \"fast\" \"safe\" ];"));
        assert!(!output.contains("modeType = types.submodule {"));
    }
}
