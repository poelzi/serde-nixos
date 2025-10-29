# Testing Guide for serde-nixos

This document describes the comprehensive testing infrastructure for serde-nixos, including unit tests, integration tests, and NixOS VM tests.

## Test Structure

### 1. Rust Unit Tests
Location: `tests/integration/`

Standard Rust tests that verify:
- Type mapping correctness
- Attribute parsing
- Doc comment extraction
- Named type generation
- All mkOption attributes

Run with:
```bash
cargo test
```

### 2. Example Tests
Location: `examples/`

Executable examples that demonstrate:
- Simple configuration
- Complex nested structures
- Advanced features (all mkOption attributes)
- Full module generation

Run with:
```bash
cargo run --example simple_config
cargo run --example complex_config
cargo run --example advanced_features
```

### 3. NixOS Integration Test
Location: `integration-test/`

**This is the crown jewel** - a real NixOS VM test that:
1. Generates NixOS module from Rust types
2. Configures a service using the generated module
3. Starts the service in a NixOS VM
4. Validates that all configuration values were passed correctly

## Running Tests

### Quick Test (Rust only)
```bash
cargo test --all
```

### With Nix Flake

#### Run all checks (including NixOS VM test)
```bash
nix flake check
```

This runs:
- ✅ Rust unit tests
- ✅ Example compilation
- ✅ Test service build
- ✅ NixOS VM integration test

#### Run just the NixOS integration test
```bash
nix build .#checks.x86_64-linux.nixos-integration
```

#### Development shell
```bash
nix develop
```

Provides:
- Rust toolchain (cargo, rustc, rust-analyzer)
- clippy, rustfmt
- cargo-watch

## Integration Test Details

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Rust Types (integration-test/src/main.rs)               │
│    - ServerConfig                                           │
│    - DatabaseConfig                                         │
│    - TestServiceConfig                                      │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ derive(NixosType)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Generated NixOS Module (integration-test/module.nix)    │
│    - serverConfigType = types.submodule { ... }            │
│    - databaseConfigType = types.submodule { ... }          │
│    - options.services.test-service = { ... }               │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ imported by
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. NixOS VM Test (integration-test/nixos-test.nix)        │
│    - Configures service using generated module             │
│    - Sets specific values                                  │
│    - Starts systemd service                                │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ validates
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Test Service Binary (test-service)                      │
│    - Reads JSON config from systemd                        │
│    - Validates all values match expected                   │
│    - Returns exit code 0 on success                        │
└─────────────────────────────────────────────────────────────┘
```

### Test Configuration

The VM test configures these values:
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

### Validation Steps

The test performs these validations:

1. **Service Starts**: Waits for systemd unit to be active
2. **Config File Generated**: Verifies systemd service has config file
3. **Values Match**: Runs `test-service --validate` to compare:
   - Expected values (from NixOS config)
   - Actual values (from generated JSON)
4. **Journal Output**: Checks service logs contain expected values:
   - Service Name: integration-test
   - Address: 0.0.0.0:3000
   - Host: db.example.com
   - Database: testdb
   - Debug: true
   - Level: debug

### Test Binaries

#### `test-service`
Main service binary that:
- Reads configuration from JSON file
- Can run in normal mode (prints config)
- Can run in validation mode (compares with expected)

Usage:
```bash
# Normal mode
test-service config.json

# Validation mode
test-service --validate actual.json expected.json
```

#### `generate-module`
Utility to regenerate the NixOS module:
```bash
cargo run --bin generate-module > integration-test/module.nix
```

## What Gets Tested

### Type Safety
- ✅ Rust types → NixOS types conversion
- ✅ Nested structures (submodules)
- ✅ Optional fields (nullOr)
- ✅ Collections (listOf, attrsOf)

### Attributes
- ✅ Doc comments → descriptions
- ✅ All mkOption attributes
- ✅ Default values
- ✅ Type validation

### Integration
- ✅ NixOS can import generated module
- ✅ Configuration values pass through correctly
- ✅ JSON serialization works
- ✅ Service starts and validates config

### End-to-End Flow
```
Rust derive → NixOS module → VM config → JSON → Rust validation
     ✓             ✓              ✓         ✓          ✓
```

## Manual Testing

### Test the service locally
```bash
# Build
cargo build --bin test-service

# Create test config
cat > /tmp/test.json << EOF
{
  "service_name": "test",
  "server": {
    "enable": true,
    "port": 3000,
    "bind_address": "0.0.0.0",
    "max_connections": 500
  },
  "database": {
    "host": "localhost",
    "port": 5432,
    "database": "testdb",
    "ssl": false
  },
  "debug": false,
  "log_level": "info"
}
EOF

# Run
./target/debug/test-service /tmp/test.json

# Validate
./target/debug/test-service --validate /tmp/test.json /tmp/test.json
```

### Regenerate NixOS module
```bash
cargo run --bin generate-module > integration-test/module.nix
```

### Run NixOS test with verbose output
```bash
nix build .#checks.x86_64-linux.nixos-integration --print-build-logs
```

## CI/CD Integration

The flake checks can be used in CI:

```yaml
# GitHub Actions example
- name: Run tests
  run: nix flake check --print-build-logs
```

All checks must pass:
- Rust tests
- Examples compile
- Test service builds
- NixOS integration test succeeds

## Troubleshooting

### "Test service failed to start"
Check the journal output in the test logs. The service might be failing to parse the config.

### "Config validation failed"
The test will print both expected and actual configs. Check for:
- Type mismatches
- Missing fields
- Incorrect default values

### "Module import failed"
Regenerate the module:
```bash
cargo run --bin generate-module > integration-test/module.nix
```

### Running tests in isolation

```bash
# Just Rust tests
nix build .#checks.x86_64-linux.rust-tests

# Just example compilation
nix build .#checks.x86_64-linux.examples-compile

# Just NixOS test
nix build .#checks.x86_64-linux.nixos-integration
```

## Success Criteria

A successful test run shows:

```
✓ Rust tests: 2 passed
✓ Examples compile
✓ Test service builds
✓ NixOS VM test:
  ✓ Service started
  ✓ Config file created
  ✓ Values validated
  ✓ Journal output correct
```

## Future Enhancements

Potential additions to the test suite:
- [ ] Test with more complex nested types
- [ ] Test enum configurations
- [ ] Test with actual network services
- [ ] Property-based testing (QuickCheck)
- [ ] Performance benchmarks
- [ ] Multiple NixOS versions
