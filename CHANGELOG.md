# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-29

### Added
- Initial implementation of `#[derive(NixosType)]` proc macro
- Support for basic Rust types (primitives, String, bool, numeric types)
- Support for `Option<T>` mapping to `types.nullOr`
- Support for `Vec<T>` mapping to `types.listOf`
- Support for `HashMap<K, V>` mapping to `types.attrsOf`
- Support for nested structs with automatic submodule generation
- Support for enums (unit variants) mapping to `types.enum`
- **Recursive nested type collection** - `nixos_type_full_definition()` now properly generates let bindings for all nested custom types
- Field-level attributes:
  - `#[nixos(description = "...")]` for option descriptions
  - `#[nixos(default = "...")]` for default values
  - `#[nixos(example = "...")]` for example values
- Struct-level `#[nixos(auto_doc)]` attribute to automatically extract doc comments
- Comprehensive integration tests covering all major features (100+ tests)
- NixOS VM integration test validating end-to-end functionality
- Generated NixOS module examples
- Nix flake with development environment and checks
- Pre-commit hooks for code quality (rustfmt, nixpkgs-fmt)
- Consolidated `rust-ci` package for comprehensive CI checks
- MIT and Apache-2.0 dual licensing
- Professional GitHub Actions CI/CD workflows
- Dependabot configuration for automated dependency updates

### Documentation
- README with quickstart guide and examples
- AGENTS.md for AI agent development workflow
- Comprehensive API documentation
- Multiple working examples in `examples/` directory
- NixOS integration test demonstrating real-world usage

### Testing
- Unit tests for core functionality
- Integration tests for:
  - Basic types
  - Nested structures
  - Enum types
  - Advanced features (defaults, examples, descriptions)
  - Auto-doc functionality
  - Enums with data (tuple and struct variants)
  - HashMap with various key types
  - Recursive and self-referential types
  - Special characters in descriptions and field names
  - Edge cases (empty structs, tuple structs, unit structs)
- NixOS VM test validating generated modules work in real NixOS systems

### Infrastructure
- Cargo workspace with separate macro and library crates
- Nix flake with:
  - Development shell with all required tools
  - Comprehensive checks (rust-ci, nix-fmt, nixos-integration, pre-commit)
  - Package outputs for all binaries
  - Formatter for code formatting
- Pre-commit configuration for code quality enforcement

### Fixed
- `nixos_type_full_definition()` now properly generates recursive let bindings for all nested custom types
- Handles types nested within `Option<T>`, `Vec<T>`, `HashMap<K, V>`, `Box<T>`, `Rc<T>`, `Arc<T>`
- Added support for `serde_json::Value` type to avoid compilation errors

### Known Limitations

#### Unsupported Features
- Enums with data variants (tuple and struct variants) have limited support
- Union types are not supported
- Generics are not yet fully supported
- Complex HashMap keys (non-String) may have limited Nix representation

#### Validation
- Generated Nix expressions are not validated for correctness
- No syntax checking of default/example values
- Users must manually verify generated NixOS modules

### Future Plans
- Complete recursive type definition generation
- Enhanced enum support (data variants, attributes)
- Generic type support
- Nix expression validation
- Better error messages for unsupported constructs
- More comprehensive type mappings
- Attribute macros for module-level configuration

---

## Release Process

This project follows semantic versioning:
- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions
- **PATCH** version for backwards-compatible bug fixes

### Version Numbering
- Development: `0.x.y` - API may change between minor versions
- Stable: `1.x.y` - Stable API with semantic versioning guarantees

### What Triggers a Version Bump
- **Major (1.0.0 → 2.0.0):**
  - Breaking changes to proc macro attributes
  - Breaking changes to generated NixOS module structure
  - Removal of public APIs
  
- **Minor (0.1.0 → 0.2.0):**
  - New features (new attributes, type support)
  - New public APIs
  - Backwards-compatible enhancements
  
- **Patch (0.1.0 → 0.1.1):**
  - Bug fixes
  - Documentation improvements
  - Internal refactoring
  - Performance improvements

[Unreleased]: https://github.com/serde-nixos/serde-nixos/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/serde-nixos/serde-nixos/releases/tag/v0.1.0
