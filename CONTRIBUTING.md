# Contributing to serde-nixos

Thank you for your interest in contributing to serde-nixos! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Submitting Changes](#submitting-changes)
- [Review Process](#review-process)

## Code of Conduct

This project follows the Rust Code of Conduct. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- **Rust:** Latest stable version (install via [rustup](https://rustup.rs/))
- **Nix:** Nix with flakes enabled (see [NixOS Wiki](https://nixos.wiki/wiki/Flakes))
- **Git:** For version control

### Ways to Contribute

- **Bug Reports:** Found a bug? Open an issue with reproduction steps
- **Feature Requests:** Have an idea? Discuss it in an issue first
- **Documentation:** Improve docs, add examples, fix typos
- **Code:** Implement features, fix bugs, improve performance
- **Tests:** Add test coverage, especially edge cases
- **Reviews:** Review pull requests, provide feedback

## Development Setup

### Clone the Repository

```bash
git clone https://github.com/serde-nixos/serde-nixos.git
cd serde-nixos
```

### Using Nix (Recommended)

Enter the development environment:

```bash
nix develop
```

This provides:
- Rust toolchain (cargo, rustc, rustfmt, clippy)
- Nix tools (nixpkgs-fmt)
- All required dependencies

### Without Nix

Install dependencies manually:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Build
cargo build
```

### Project Structure

```
serde-nixos/
├── serde-nixos-macros/     # Proc macro implementation
│   └── src/
│       ├── lib.rs          # Entry point
│       ├── nixos_type.rs   # Core macro logic
│       ├── attributes.rs   # Attribute parsing
│       └── type_mapping.rs # Rust→Nix type mapping
│
├── serde-nixos/            # Public API library
│   └── src/
│       ├── lib.rs          # Public exports
│       └── generator.rs    # Utility functions
│
├── integration-test/       # NixOS VM integration test
│   ├── src/
│   │   ├── config.rs       # Shared config types
│   │   ├── main.rs         # Test service
│   │   └── generate_module.rs
│   └── nixos-test.nix      # VM test definition
│
├── tests/integration/      # Integration tests
├── examples/               # Usage examples
└── flake.nix              # Nix flake
```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

Branch naming:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `test/` - Test additions/improvements
- `refactor/` - Code refactoring

### 2. Make Your Changes

Follow the coding style and conventions in existing code.

### 3. Write Tests

All code changes must include tests:

- **New features:** Add integration tests in `tests/integration/`
- **Bug fixes:** Add regression tests
- **Refactoring:** Ensure existing tests still pass

### 4. Update Documentation

- Add/update doc comments for public APIs
- Update README.md if user-facing behavior changes
- Add examples for new features
- Update CHANGELOG.md under `[Unreleased]`

## Testing

### Run All Tests

```bash
cargo test --all
```

### Run Specific Test File

```bash
cargo test --test basic_types
```

### Run Specific Test

```bash
cargo test test_auto_doc_enabled
```

### With Output

```bash
cargo test -- --nocapture
```

### NixOS Integration Test

```bash
nix build .#checks.x86_64-linux.nixos-integration
```

### Test Coverage Goals

- Unit tests: >80% coverage of core logic
- Integration tests: All major features
- Edge cases: Empty inputs, special characters, recursive types

## Code Quality

### Format Code

Always format before committing:

```bash
# Format Rust
cargo fmt --all

# Format Nix
nixpkgs-fmt flake.nix integration-test/*.nix
```

### Run Clippy

Fix all clippy warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Auto-fix some issues:

```bash
cargo clippy --fix --all-targets --all-features
```

### Run All Checks

Before submitting, ensure all checks pass:

```bash
# Format check
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all

# Examples
cargo build --examples

# Full flake checks (if Nix available)
nix flake check
```

### Pre-commit Hooks

We use pre-commit hooks for code quality. Install them:

```bash
# If using Nix
nix develop
# Hooks are automatically installed in the shell

# Manual installation
cargo install pre-commit
pre-commit install
```

The hooks run:
- `rustfmt` on Rust files
- `nixpkgs-fmt` on Nix files

## Submitting Changes

### Commit Messages

Write clear, descriptive commit messages:

```
<type>: <short summary>

<detailed description>

<footer>
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `test:` Adding tests
- `refactor:` Code refactoring
- `perf:` Performance improvement
- `chore:` Maintenance tasks

**Example:**

```
feat: add support for tuple struct variants

Implement NixOS type generation for enums with tuple variants.
The generated type wraps each variant in an appropriate structure
that can be serialized to Nix.

- Add variant data parsing
- Generate types.attrs with variant discrimination
- Add comprehensive tests for various tuple patterns
- Update documentation with examples

Closes #42
```

### Guidelines

- Keep commits atomic (one logical change per commit)
- Reference issue numbers in commit messages
- Write descriptive commit messages
- Keep the first line under 72 characters

### Create Pull Request

1. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Open a pull request on GitHub

3. Fill out the PR template:
   - Describe the changes
   - Reference related issues
   - Explain testing performed
   - Note any breaking changes

4. Ensure CI passes

### PR Checklist

Before submitting, verify:

- [ ] All tests pass (`cargo test --all`)
- [ ] Code is formatted (`cargo fmt --all -- --check`)
- [ ] No clippy warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Examples build (`cargo build --examples`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commits are clean and descriptive
- [ ] Branch is up to date with main

## Review Process

### What to Expect

- Maintainers will review your PR within a few days
- You may receive feedback and change requests
- Discussion and iteration are normal and welcomed
- Once approved, a maintainer will merge your PR

### Responding to Feedback

- Address all comments
- Push additional commits or amend existing ones
- Mark conversations as resolved when addressed
- Ask questions if feedback is unclear

### After Merge

- Your changes will be in the next release
- You'll be credited in the changelog
- Thank you for contributing! 🎉

## Development Tips

### Debugging Macros

Use `cargo-expand` to see macro output:

```bash
cargo install cargo-expand
cargo expand --test auto_doc test_auto_doc_enabled
```

### Running Examples

```bash
cargo run --example auto_doc_example
```

### Documentation

Generate and view docs:

```bash
cargo doc --open
```

### Benchmarking

```bash
cargo bench
```

## Style Guide

### Rust Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` defaults (Rust 2021 edition)
- Prefer explicit types in public APIs
- Avoid `unwrap()` in library code (use `?` or `expect()` with clear messages)
- Use meaningful variable names
- Keep functions focused and under 100 lines
- Add doc comments to all public items

### Documentation Style

- Start with a brief one-line summary
- Include examples for public APIs
- Explain "why" not just "what"
- Use code blocks with ```rust
- Link to related items with `[`item`]`

Example:

```rust
/// Generates NixOS type definitions from Rust structures.
///
/// This derive macro automatically creates NixOS module definitions,
/// ensuring type safety between Rust and NixOS configurations.
///
/// # Examples
///
/// ```rust
/// use serde_nixos::NixosType;
///
/// #[derive(NixosType)]
/// struct Config {
///     port: u16,
/// }
/// ```
#[proc_macro_derive(NixosType, attributes(nixos))]
pub fn derive_nixos_type(input: TokenStream) -> TokenStream {
    // ...
}
```

### Test Style

- Use descriptive names: `test_feature_behavior`
- Test one thing per test
- Include both positive and negative cases
- Add comments explaining complex test setup
- Use helper functions for repeated setup

### Commit Style

- Present tense: "Add feature" not "Added feature"
- Imperative mood: "Fix bug" not "Fixes bug"
- Capitalize first word
- No period at end of subject line
- Blank line between subject and body
- Wrap body at 72 characters

## Getting Help

- **Questions:** Open a GitHub discussion or issue
- **Chat:** (Add chat link if available)
- **Documentation:** See README.md and docs/
- **Issues:** Search existing issues first

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to serde-nixos! Your efforts help make NixOS configuration more type-safe and developer-friendly.
