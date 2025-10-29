# Agent Development Workflow

This document provides instructions for AI agents (Claude, etc.) working on the serde-nixos codebase.

## Project Overview

**serde-nixos** is a Rust procedural macro library that generates NixOS type definitions from Rust structures. It bridges the gap between Rust configuration types and NixOS module system, ensuring type safety across both ecosystems.

**Languages:** Rust, Nix
**Build System:** Cargo (Rust), Nix Flakes
**Testing:** Rust unit/integration tests, NixOS VM tests

## ⚠️ Critical Development Rules

### NEVER Remove Tests or Checks Without Permission

**This is a hard rule. Violations are not acceptable.**

1. **Do NOT remove or disable tests** - Ever. If a test is failing:
   - Fix the code to make the test pass, OR
   - Fix the test if it's incorrectly written, OR
   - Ask the user for guidance if the test represents outdated requirements

2. **Do NOT remove checks from `nix flake check`** - The flake checks exist for a reason:
   - `rust-ci` - Comprehensive Rust check package that runs:
     - Code formatting (cargo fmt --check)
     - Linting (cargo clippy with warnings as errors)
     - All tests (cargo test --all)
     - Workspace build (cargo build --all --release)
     - Examples build (cargo build --examples --release)
   - `nix-fmt` - Ensures Nix code is formatted
   - `test-service-builds` - Ensures integration test service builds
   - `nixos-integration` - Ensures NixOS VM test passes
   - `pre-commit` - Ensures pre-commit hooks work (formatting only)

3. **If checks fail in Nix sandbox** - Don't remove them. Instead:
   - Ask the user about the intended behavior
   - Propose alternative solutions
   - Fix the underlying issue
   - NEVER silently remove checks

4. **When tempted to remove checks** - STOP and ask:
   - "Why is this check failing?"
   - "What is the check protecting against?"
   - "What would be the risk if this check didn't exist?"
   - "Should I ask the user before removing this?"

5. **Always ask first** - If you believe a test or check should be removed:
   - Explain WHY you think it should be removed
   - Describe what the check was doing
   - Explain the implications of removing it
   - Wait for explicit user approval

**Remember:** Tests and checks are documentation of requirements. Removing them without permission is equivalent to changing requirements without approval.

### Keep Configuration Files Minimal

When creating or updating configuration files (.gitattributes, .editorconfig, etc.):

1. **Only include file types that actually exist in the project**
   - Don't add definitions for .json if project has no JSON files
   - Don't add rules for .sh/.bash if project has no shell scripts
   - Keep it minimal and relevant

2. **Check what file types exist first**
   ```bash
   find . -type f -not -path '*/\.*' -not -path '*/target/*' | sed 's/.*\.//' | sort | uniq
   ```

3. **This project uses:**
   - Rust: `.rs`
   - Nix: `.nix`
   - Cargo: `.toml`, `.lock`
   - Documentation: `.md`
   - Licenses: `LICENSE-MIT`, `LICENSE-APACHE`
   - GitHub Actions: `.yml`
   
   Don't add rules for other file types unless they're actually added to the project.

## Code Quality Standards

All code contributions must meet these quality standards:

### 1. Rust Code Quality

#### Formatting
- **Tool:** `rustfmt`
- **Standard:** Rust 2021 edition formatting
- **Command:** `cargo fmt --all`
- **CI Check:** `cargo fmt --all -- --check`

#### Linting
- **Tool:** `clippy`
- **Standard:** All clippy warnings must be addressed
- **Command:** `cargo clippy --all-targets --all-features`
- **Allowed:** Only explicitly documented exceptions in code
- **CI Check:** `cargo clippy --all-targets --all-features -- -D warnings`

#### Code Style
- Use meaningful variable names
- Add doc comments (`///`) for all public items
- Keep functions focused and under 100 lines
- Prefer explicit types over inference in public APIs
- Use `Result` and `Option` properly, avoid `unwrap()` in library code

### 2. Nix Code Quality

#### Formatting
- **Tool:** `nixfmt` or `nixpkgs-fmt`
- **Standard:** Nixpkgs style guide
- **Command:** `nixfmt flake.nix integration-test/*.nix`
- **Consistency:** All Nix files must be formatted

#### Style
- Use `let...in` for local bindings
- Prefer `mkOption` with explicit types
- Document all NixOS options
- Keep expressions readable (break long lines)

### 3. Documentation

All code must be documented:

- **Public APIs:** Full doc comments with examples
- **Modules:** Module-level documentation
- **Complex Logic:** Inline comments explaining "why"
- **Tests:** Test names that describe what's being tested
- **Examples:** Working, tested examples for features

## Testing Requirements

### Unit Tests

**Location:** `serde-nixos/src/`, `serde-nixos-macros/src/`
**Command:** `cargo test --lib`

Requirements:
- Test all public functions
- Test edge cases (empty inputs, invalid data, etc.)
- Use descriptive test names: `test_feature_behavior`
- Include both positive and negative test cases

Example:
```rust
#[test]
fn test_auto_doc_enabled() {
    #[derive(NixosType)]
    #[nixos(auto_doc)]
    struct Config {
        /// Field doc
        field: String,
    }

    let opts = Config::nixos_options();
    assert!(opts.contains("description = \"Field doc\""));
}
```

### Integration Tests

**Location:** `tests/integration/`
**Command:** `cargo test --test <test_name>`

Requirements:
- Test feature combinations
- Test real-world usage scenarios
- Test interaction between components
- Must be in separate files in `tests/integration/`

Categories:
- `basic_types.rs` - Primitive type mapping
- `nested_structs.rs` - Complex nested structures
- `enum_types.rs` - Enum handling
- `advanced_features.rs` - All mkOption attributes
- `auto_doc.rs` - Auto-doc feature

### NixOS Integration Tests

**Location:** `integration-test/nixos-test.nix`
**Command:** `nix build .#checks.x86_64-linux.nixos-integration`

Requirements:
- Test generated NixOS modules work in real VMs
- Verify configuration values pass through correctly
- Test systemd service integration
- Validate end-to-end workflow

This is the **proof** that the library actually works with NixOS.

### Test Coverage Goals

- **Unit Tests:** >80% coverage of core logic
- **Integration Tests:** All major features
- **NixOS Tests:** Critical path (config generation → NixOS → validation)

## Pre-commit Checks

Before committing, all these checks must pass:

```bash
# Format Rust code
cargo fmt --all

# Format Nix code
nixfmt flake.nix integration-test/*.nix

# Lint Rust code
cargo clippy --all-targets --all-features

# Run all tests
cargo test --all

# Build examples
cargo build --examples

# Run flake checks
nix flake check
```

## Development Workflow

### Making Changes

1. **Understand the feature**
   - Read existing code and documentation
   - Check related tests
   - Review examples

2. **Plan the implementation**
   - Identify affected modules
   - Consider backward compatibility
   - Plan test coverage

3. **Implement**
   - Write failing tests first (TDD)
   - Implement the feature
   - Make tests pass
   - Add documentation

4. **Quality checks**
   - Run `cargo fmt --all`
   - Run `cargo clippy --all-targets --all-features`
   - Ensure all tests pass: `cargo test --all`
   - Build examples: `cargo build --examples`
   - Run flake checks: `nix flake check`

5. **Documentation**
   - Add/update doc comments
   - Update relevant .md files
   - Add examples if needed
   - Update CHANGELOG (if exists)

### File Organization

```
serde-nixos/
├── serde-nixos-macros/      # Proc macro implementation
│   ├── src/
│   │   ├── lib.rs           # Entry point
│   │   ├── nixos_type.rs    # Core macro logic
│   │   ├── attributes.rs    # Attribute parsing
│   │   └── type_mapping.rs  # Rust→Nix type mapping
│   └── Cargo.toml
│
├── serde-nixos/             # Main library
│   ├── src/
│   │   ├── lib.rs           # Public API
│   │   └── generator.rs     # Utilities
│   └── Cargo.toml
│
├── integration-test/        # NixOS VM test
│   ├── src/
│   │   ├── main.rs          # Test service
│   │   └── generate_module.rs
│   ├── module.nix           # Generated module
│   └── nixos-test.nix       # VM test
│
├── tests/integration/       # Integration tests
├── examples/                # Usage examples
├── flake.nix               # Nix flake
└── docs/                   # Documentation
```

### Adding a New Feature

**Step-by-step process:**

1. **Write tests first**
   ```bash
   # Create test file
   vim tests/integration/new_feature.rs

   # Add to Cargo.toml
   [[test]]
   name = "new_feature"
   path = "../tests/integration/new_feature.rs"
   ```

2. **Implement the feature**
   ```bash
   # Edit relevant files
   vim serde-nixos-macros/src/nixos_type.rs
   ```

3. **Add example**
   ```bash
   vim examples/new_feature_example.rs
   ```

4. **Document**
   ```bash
   # Update or create docs
   vim NEW_FEATURE.md
   # Update main README
   vim README.md
   ```

5. **Run all checks**
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features
   cargo test --all
   cargo build --examples
   nix flake check  # If available
   ```

## Flake Checks

The `nix flake check` command runs comprehensive checks:

### Consolidated Rust CI Check
- ✅ `rust-ci` - Single comprehensive Rust package built with `rustPlatform.buildRustPackage`
  - **Usage:** `nix build .#checks.x86_64-linux.rust-ci` or `nix flake check`
  - **Note:** This is a check derivation, not a runnable package. Use `nix build`, NOT `nix run`.
  - **Benefits:** Proper cargo dependency vendoring, single build, better Nix caching
  - **Includes:**
    - Code formatting check (`cargo fmt --all -- --check`)
    - Linting with errors (`cargo clippy --all-targets --all-features -- -D warnings`)
    - All unit and integration tests (`cargo test --all --release`)
    - Full workspace build (`cargo build --all --release`)
    - All examples build (`cargo build --examples --release`)
    - Documentation generation (`cargo doc --all --no-deps --document-private-items`)
  - **Output:** Binaries installed to `$out/bin` (test-service, generate-module)

### Additional Checks
- ✅ `nix-fmt` - Nix files are formatted (nixpkgs-fmt)
- ✅ `test-service-builds` - Integration test service builds standalone
- ✅ `nixos-integration` - NixOS VM test passes (end-to-end validation)
- ✅ `pre-commit` - Pre-commit hooks work (rustfmt, nixpkgs-fmt only)

### Why Consolidated CI Check?

Previously, we had separate `runCommand` checks for rust-fmt, rust-clippy, rust-tests, and examples-compile. 
These were replaced with a single `rust-ci` package for several reasons:

1. **Proper Vendoring:** `rustPlatform.buildRustPackage` automatically handles cargo dependency vendoring via `cargoLock.lockFile`
2. **Nix Sandbox Compatibility:** Works correctly in offline Nix sandbox builds
3. **Better Caching:** Single derivation with proper Nix caching of dependencies
4. **Faster CI:** One build instead of multiple separate command invocations
5. **Standard Practice:** Follows Nix/Rust ecosystem conventions

All checks must pass before code is considered ready.

## Common Tasks

### Running Tests

```bash
# All tests
cargo test --all

# Specific test file
cargo test --test auto_doc

# Specific test
cargo test test_auto_doc_enabled

# With output
cargo test -- --nocapture

# NixOS test
nix build .#checks.x86_64-linux.nixos-integration
```

### Formatting

```bash
# Format Rust
cargo fmt --all

# Check Rust formatting
cargo fmt --all -- --check

# Format Nix (if nixfmt available)
nixfmt flake.nix
nixfmt integration-test/*.nix
```

### Linting

```bash
# Run clippy
cargo clippy --all-targets --all-features

# Clippy with warnings as errors
cargo clippy --all-targets --all-features -- -D warnings

# Auto-fix some clippy issues
cargo clippy --fix --all-targets --all-features
```

### Building

```bash
# Build library
cargo build

# Build with all features
cargo build --all-features

# Build examples
cargo build --examples

# Build specific example
cargo build --example auto_doc_example

# Build for release
cargo build --release
```

### Documentation

```bash
# Generate Rust docs
cargo doc --open

# Generate docs for dependencies too
cargo doc --open --document-private-items
```

## Debugging

### Macro Debugging

To see expanded macro output:
```bash
# Install cargo-expand
cargo install cargo-expand

# Expand a specific test
cargo expand --test auto_doc test_auto_doc_enabled
```

### Test Debugging

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Run with full backtrace
RUST_BACKTRACE=full cargo test

# Run specific test with output
cargo test test_auto_doc_enabled -- --nocapture
```

### NixOS Test Debugging

```bash
# Build with logs
nix build .#checks.x86_64-linux.nixos-integration --print-build-logs

# Keep failed build for inspection
nix build .#checks.x86_64-linux.nixos-integration --keep-failed
```

## Error Handling

### In Library Code
- Use `Result<T, E>` for fallible operations
- Use descriptive error types
- Avoid `panic!`, `unwrap()`, and `expect()` in library code
- Use `?` operator for error propagation

### In Tests
- `assert!`, `assert_eq!`, `assert_ne!` are appropriate
- Use `unwrap()` or `expect()` with clear messages
- Test both success and error cases

### In Examples
- Can use `unwrap()` with comments explaining
- Show error handling patterns for users

## Performance Considerations

- Proc macros should be fast (users run them at compile time)
- Avoid unnecessary allocations
- Use `&str` over `String` where possible
- Profile with `cargo bench` for critical paths

## Dependencies

### Adding Dependencies

1. Add to workspace `Cargo.toml`:
   ```toml
   [workspace.dependencies]
   new-dep = "1.0"
   ```

2. Use in package `Cargo.toml`:
   ```toml
   [dependencies]
   new-dep = { workspace = true }
   ```

3. Prefer minimal dependencies
4. Check license compatibility (MIT/Apache-2.0)

## Git Workflow

### Commit Messages

Format:
```
<type>: <subject>

<body>

<footer>
```

Types:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Tests
- `refactor:` Code refactoring
- `chore:` Maintenance

Example:
```
feat: add auto_doc attribute for automatic doc comment extraction

Implement #[nixos(auto_doc)] struct-level attribute that automatically
uses Rust doc comments as NixOS option descriptions. This eliminates
duplication between doc comments and #[nixos(description)] attributes.

- Add NixosStructAttributes for struct-level config
- Update attribute parsing and combination logic
- Add 10 comprehensive tests
- Add example and documentation

Closes #123
```

## Release Checklist

When preparing a release:

1. ✅ All tests pass (`cargo test --all`)
2. ✅ All examples build (`cargo build --examples`)
3. ✅ Flake check passes (`nix flake check`)
4. ✅ Documentation is up to date
5. ✅ CHANGELOG is updated
6. ✅ Version numbers bumped
7. ✅ `cargo publish --dry-run` succeeds

## Resources

### Documentation
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [NixOS Manual](https://nixos.org/manual/nixos/stable/)
- [Nix Pills](https://nixos.org/guides/nix-pills/)

### Tools
- [cargo-expand](https://github.com/dtolnay/cargo-expand) - Expand macros
- [cargo-watch](https://github.com/watchexec/cargo-watch) - Auto-rebuild on changes
- [rust-analyzer](https://rust-analyzer.github.io/) - LSP for Rust

## Agent-Specific Notes

When working as an AI agent:

1. **Always run tests** - Don't assume code works
2. **Check existing patterns** - Follow established code style
3. **Update documentation** - Code without docs is incomplete
4. **Consider backward compatibility** - Don't break existing users
5. **Ask for clarification** - If requirements are unclear
6. **Test incrementally** - Verify each change works
7. **Use flake checks** - `nix flake check` is the final arbiter
8. **Use AGENTS.md only** - NEVER CREATE OR CHANGE CLAUDE.md

IMPORTANT: run commands always using `nix flake develop --command`

## Summary

**Code Quality Formula:**
```
rustfmt + clippy + tests + docs + nix flake check = ✅
```

Every contribution must:
- ✅ Be formatted (rustfmt, nixfmt)
- ✅ Pass clippy with no warnings
- ✅ Have tests (unit + integration)
- ✅ Be documented
- ✅ Pass `nix flake check`

Follow these guidelines, and the codebase will remain high quality, maintainable, and reliable.
