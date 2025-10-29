//! Generator utilities for creating NixOS modules from Rust types

use std::fmt::Write;

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
