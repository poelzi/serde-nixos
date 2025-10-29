# Integration Test for serde-nixos

This directory contains a * *real NixOS integration test * * that validates the entire serde-nixos workflow
  end-to-end.## What This Tests

  This
  integration
  test
  proves that:
  1. ✅ Rust types can be converted to valid NixOS modules
  2. ✅
  Generated
  NixOS
  modules
  can
  be
  imported
  and
  used
  3. ✅
  Configuration
  values
  set in NixOS correctly reach the Rust application
4. ✅ The entire workflow is type-safe from Rust → Nix → JSON → Rust

## Components

### `src/main.rs` - Test Service
A simple service that:
- Reads configuration from a JSON file
- Validates that values match expectations
- Has two modes:
- **Normal mode**: Print configuration and exit
- **Validation mode**: Compare actual vs expected config

### `src/generate_module.rs` - Module Generator
Generates the NixOS module from the Rust types defined in `main.rs`.

Run with:
```bash
cargo run --bin generate-module > module.nix
```

### `module.nix` - Generated NixOS Module
**Auto-generated** NixOS module that defines:
- Type definitions (serverConfigType, databaseConfigType, etc.)
- options.services.test-service with full type safety
- systemd service configuration

### `nixos-test.nix` - NixOS VM Test
The actual integration test that:
1. Creates a NixOS VM
2. Imports the generated module
3. Configures the service with specific values
4. Starts the service
5. Validates all values were passed correctly

## Running the Test

### Via Nix Flake (Recommended)
```bash
# From repository root
nix flake check

# Or run just this test
nix build .#checks.x86_64-linux.nixos-integration
```

### Manually

```bash
# 1. Generate the module
cargo run --bin generate-module > module.nix

# 2. Build the test service
cargo build --release --bin test-service

# 3. Run the NixOS test
nix-build nixos-test.nix
```

## Test Configuration

The test configures these values in NixOS:

```nix
services.test-service = {
enable = true;
service_name = "integration-test";

server = {
enable = true;
port = 3000;
bind_address = "0.0.0.0";
max_connections = 500;
};

database = {
host = "db.example.com";
port = 5432;
database = "testdb";
ssl = true;
};

debug = true;
log_level = "debug";
};
```

## What Gets Validated

The test service validates:
- ✅ service_name = "integration-test"
- ✅ server.port = 3000
- ✅ server.bind_address = "0.0.0.0"
- ✅ server.max_connections = 500
- ✅ database.host = "db.example.com"
- ✅ database.port = 5432
- ✅ database.database = "testdb"
- ✅ database.ssl = true
- ✅ debug = true
- ✅ log_level = "debug"

## Test Flow

```
┌──────────────────────────┐
│ 1. Define Rust Types     │
│    with #[derive(       │
│      NixosType)]         │
└────────────┬─────────────┘
│
▼
┌──────────────────────────┐
│ 2. Generate NixOS Module │
│    cargo run --bin       │
│    generate-module       │
└────────────┬─────────────┘
│
▼
┌──────────────────────────┐
│ 3. NixOS VM imports      │
│    generated module      │
└────────────┬─────────────┘
│
▼
┌──────────────────────────┐
│ 4. Configure service     │
│    with test values      │
└────────────┬─────────────┘
│
▼
┌──────────────────────────┐
│ 5. NixOS generates JSON  │
│    config file from      │
│    options               │
└────────────┬─────────────┘
│
▼
┌──────────────────────────┐
│ 6. Systemd starts        │
│    test-service with     │
│    config file           │
└────────────┬─────────────┘
│
▼
┌──────────────────────────┐
│ 7. Service reads JSON    │
│    and validates all     │
│    values match          │
└────────────┬─────────────┘
│
▼
✓ SUCCESS!
```

## Expected Output

When the test passes, you'll see:

```
✓ Configuration validation PASSED
All values match expected configuration:
Service: integration-test
Server: 0.0.0.0:3000
Database: db.example.com:5432/testdb
Debug: true
Log Level: debug
```

## Files

- `Cargo.toml` - Package definition
- `Cargo.lock` - Locked dependencies (checked in for Nix)
- `src/main.rs` - Test service implementation
- `src/generate_module.rs` - Module generator
- `module.nix` - **Generated** NixOS module (regenerate with generate-module)
- `nixos-test.nix` - NixOS VM test definition
- `README.md` - This file

## Regenerating the Module

The `module.nix` file is auto-generated from Rust types. To regenerate:

```bash
cargo run --bin generate-module > module.nix
```

**When to regenerate:**
- After changing types in `main.rs` or `generate_module.rs`
- After updating serde-nixos library
- After changing type attributes

## Development Workflow

1. Modify types in `src/main.rs`
2. Update `src/generate_module.rs` if needed
3. Regenerate module: `cargo run --bin generate-module > module.nix`
4. Update test configuration in `nixos-test.nix` if needed
5. Run test: `nix build .#checks.x86_64-linux.nixos-integration`

## Troubleshooting

### Module import fails
```bash
# Check module syntax
nix-instantiate --eval module.nix

# Regenerate module
cargo run --bin generate-module > module.nix
```

### Service fails to start
```bash
# Check service logs in test output
# Look for JSON parsing errors
# Verify all required fields are set
```

### Validation fails
The test will print:
- Expected configuration (from test setup)
- Actual configuration (from NixOS)
Diff them to find mismatches.

## Adding New Tests

To test additional configuration:

1. Add types to `src/main.rs`
2. Update `src/generate_module.rs` to include new types
3. Regenerate `module.nix`
4. Update `nixos-test.nix` with test configuration
5. Update expected config in `nixos-test.nix`
6. Run the test!

## This Proves

This integration test provides **concrete proof** that:
- Generated NixOS modules are syntactically valid
- Type conversions are correct (Rust → Nix)
- Configuration values flow through correctly (Nix → JSON)
- The entire stack is type-safe and works in practice
- Not just a demo - a real, working integration!
