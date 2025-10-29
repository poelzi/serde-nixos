# serde-nixos

[![Crates.io](https://img.shields.io/crates/v/serde-nixos.svg)](https://crates.io/crates/serde-nixos)
[![Documentation](https://docs.rs/serde-nixos/badge.svg)](https://docs.rs/serde-nixos)
[![CI](https://github.com/serde-nixos/serde-nixos/workflows/CI/badge.svg)](https://github.com/serde-nixos/serde-nixos/actions)
[![License](https://img.shields.io/crates/l/serde-nixos.svg)](https://github.com/serde-nixos/serde-nixos#license)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

A Rust procedural macro that automatically generates NixOS type definitions from your Rust structures using serde. This allows you to maintain a single source of truth for your configuration structures and automatically generate the corresponding NixOS module definitions.

## Features

- **Automatic Type Mapping**: Converts Rust types to their NixOS equivalents
- **Serde Integration**: Works with existing serde attributes
- **Doc Comment Support**: Automatically uses Rust doc comments as NixOS descriptions
- **Full mkOption Support**: All NixOS `lib.mkOption` attributes supported (description, default, defaultText, example, apply, internal, visible, readOnly, relatedPackages)
- **Named Type Definitions**: Generates named types like `serverConfigType = types.submodule {...}`
- **Let Chain Generation**: Creates full definitions with `let...in` chains for all dependent types
- **Nested Structure Support**: Handles complex nested structures and generates proper submodules
- **Collection Support**: Maps Vec, HashMap, and other collections to appropriate NixOS types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde-nixos = "0.1"
```

## Quick Start

```rust
use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;

#[derive(Serialize, Deserialize, NixosType)]
struct ServerConfig {
    #[nixos(description = "Enable the server")]
    enable: bool,
    
    #[nixos(description = "Port to listen on", default = "8080")]
    port: u16,
    
    #[nixos(description = "Host to bind to", default = "\"localhost\"")]
    host: String,
    
    #[nixos(description = "Maximum connections")]
    max_connections: Option<u32>,
}

fn main() {
    // Generate NixOS module definition
    let nixos_module = ServerConfig::nixos_type_definition();
    println!("{}", nixos_module);
}
```

This generates:

```nix
# NixOS module definition for ServerConfig
{
  options = {
    enable = lib.mkOption {
      type = types.bool;
      description = "Enable the server";
    };

    port = lib.mkOption {
      type = types.int;
      description = "Port to listen on";
      default = 8080;
    };

    host = lib.mkOption {
      type = types.str;
      description = "Host to bind to";
      default = "localhost";
    };

    max_connections = lib.mkOption {
      type = types.nullOr types.int;
      description = "Maximum connections";
    };
  };
}
```

## Type Mappings

The macro automatically maps Rust types to NixOS types:

| Rust Type | NixOS Type |
|-----------|------------|
| `bool` | `types.bool` |
| `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64` | `types.int` |
| `f32`, `f64` | `types.float` |
| `String`, `&str` | `types.str` |
| `PathBuf`, `Path` | `types.path` |
| `Vec<T>` | `types.listOf <T>` |
| `HashMap<K, V>`, `BTreeMap<K, V>` | `types.attrsOf <V>` |
| `Option<T>` | `types.nullOr <T>` |
| Custom structs | `types.submodule { ... }` |
| Enums | `types.enum [ ... ]` |

## Attributes

### NixOS Attributes

You can use `#[nixos(...)]` attributes to customize the generated NixOS options:

- `description = "..."` - Add a description to the option
- `default = "..."` - Set a default value (must be valid Nix syntax)
- `example = "..."` - Provide an example value
- `optional` - Make the field optional (alternative to `Option<T>`)
- `rename = "..."` - Rename the field in the NixOS module
- `skip` - Skip this field in the NixOS module

### Serde Attribute Support

The macro respects serde attributes:

- `#[serde(rename = "...")]` - Renames the field (unless overridden by `#[nixos(rename)]`)
- `#[serde(skip)]` - Skips the field
- `#[serde(default)]` - Makes the field optional in NixOS

## Complex Example

```rust
use serde::{Deserialize, Serialize};
use serde_nixos::NixosType;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, NixosType)]
struct ApplicationConfig {
    #[nixos(description = "Application name")]
    name: String,
    
    #[nixos(description = "Server configuration")]
    server: ServerConfig,
    
    #[nixos(description = "Database configuration")]
    database: DatabaseConfig,
    
    #[nixos(description = "Feature flags")]
    features: HashMap<String, bool>,
    
    #[nixos(description = "List of plugins to load")]
    plugins: Vec<PluginConfig>,
}

#[derive(Serialize, Deserialize, NixosType)]
struct ServerConfig {
    #[nixos(description = "Bind address", default = "\"0.0.0.0\"")]
    bind_address: String,
    
    #[nixos(description = "Port", default = "8080")]
    port: u16,
    
    #[nixos(description = "Use HTTPS")]
    use_https: bool,
}

#[derive(Serialize, Deserialize, NixosType)]
struct DatabaseConfig {
    #[nixos(description = "Database URL")]
    url: String,
    
    #[nixos(description = "Connection pool size", default = "10")]
    pool_size: u32,
}

#[derive(Serialize, Deserialize, NixosType)]
struct PluginConfig {
    #[nixos(description = "Plugin name")]
    name: String,
    
    #[nixos(description = "Plugin enabled")]
    enabled: bool,
}
```

## Generating Complete NixOS Modules

You can use the generated types to create complete NixOS modules with systemd services:

```rust
fn generate_nixos_module() -> String {
    format!(r#"
{{ config, lib, pkgs, ... }}:

with lib;

let
  cfg = config.services.myapp;
in
{{
  options.services.myapp = {{
{}
  }};

  config = mkIf cfg.enable {{
    systemd.services.myapp = {{
      description = "My Application";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      
      serviceConfig = {{
        ExecStart = "${{cfg.package}}/bin/myapp";
        Restart = "on-failure";
      }};
    }};
  }};
}}
"#, ApplicationConfig::nixos_options())
}
```

## CLI Tool Example

See `examples/generate_nixos.rs` for a complete example of a CLI tool that generates NixOS modules from Rust configuration structures.

## Running Examples

```bash
# Simple configuration example
cargo run --example simple_config

# Complex nested configuration
cargo run --example complex_config

# CLI tool for generating NixOS modules
cargo run --example generate_nixos
```

## Testing

Run the test suite:

```bash
cargo test
```

## Limitations

- Union types are not supported
- Tuple structs generate generic `types.attrs`
- Default values must be valid Nix syntax (strings need quotes)
- Enums only support simple variants (no associated data)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Roadmap

- [ ] Support for enum variants with associated data
- [ ] Support for custom type validators
- [ ] Generate Nix functions for serialization/deserialization
- [ ] Support for more complex default value expressions
- [ ] Support for recursive types
- [ ] Better error messages for invalid attributes
## New Features (v0.2)

### Doc Comment Extraction

Doc comments are automatically used as descriptions if no explicit `#[nixos(description)]` is provided:

```rust
#[derive(Serialize, Deserialize, NixosType)]
struct Config {
    /// The server port to listen on
    port: u16,
    
    #[nixos(description = "Explicit description takes precedence")]
    /// This doc comment will be ignored
    host: String,
}
```

### Full mkOption Attribute Support

All `lib.mkOption` attributes from NixOS are now supported:

```rust
#[derive(Serialize, Deserialize, NixosType)]
struct AdvancedConfig {
    #[nixos(
        description = "The service user",
        default = "\"myservice\"",
        example = "\"customuser\"",
        apply = "x: assert x != \"root\"; x"
    )]
    user: String,
    
    #[nixos(internal)]
    internal_state: String,
    
    #[nixos(read_only)]
    computed_value: String,
    
    #[nixos(default_text = "\"computed at runtime\"")]
    dynamic_default: String,
    
    #[nixos(visible = "false")]
    hidden_option: String,
    
    #[nixos(related_packages = "[ pkgs.foo pkgs.bar ]")]
    package_option: String,
}
```

### Named Type Definitions

Each struct now generates a named type:

```rust
#[derive(Serialize, Deserialize, NixosType)]
struct ServerConfig {
    port: u16,
}

// Generates: serverConfigType = types.submodule { ... }
println!("{}", ServerConfig::nixos_type_name());  // "serverConfigType"
println!("{}", ServerConfig::nixos_type());       // "serverConfigType"
```

### Full Definition with Let Chains

The `nixos_type_full_definition()` method generates complete definitions with all dependent types:

```rust
#[derive(Serialize, Deserialize, NixosType)]
struct DatabaseConfig {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize, NixosType)]
struct AppConfig {
    database: DatabaseConfig,
}

// Generates a let chain with all types defined
println!("{}", AppConfig::nixos_type_full_definition());
```

Output:
```nix
let
  appConfigType = types.submodule {
    options = {
      database = lib.mkOption {
        type = databaseConfigType;
        # ...
      };
    };
  };
in appConfigType
```

### API Methods

Each type that derives `NixosType` now has these methods:

- `nixos_type_definition()` - Full module definition with named type
- `nixos_options()` - Just the options portion
- `nixos_type()` - The type expression (returns the named type)
- `nixos_type_name()` - The generated type name (e.g., "serverConfigType")
- `nixos_type_full_definition()` - Full definition with `let...in` chain for dependencies

## Complete Example

See `examples/advanced_features.rs` for a comprehensive example demonstrating all features.

```bash
cargo run --example advanced_features
```


## Auto-Doc Feature (NEW!)

Automatically use Rust doc comments as NixOS descriptions with `#[nixos(auto_doc)]`:

```rust
#[derive(Serialize, Deserialize, NixosType)]
#[nixos(auto_doc)]
struct ServerConfig {
    /// The server port to listen on
    #[nixos(default = "8080")]
    port: u16,
    
    /// Enable debug logging
    #[nixos(default = "false")]
    debug: bool,
}
```

**Benefits:**
- ✅ Write documentation once, use everywhere
- ✅ No need for `#[nixos(description = "...")]` on every field
- ✅ Standard Rust doc comments work automatically
- ✅ Multi-line comments supported

See [AUTO_DOC.md](AUTO_DOC.md) for complete documentation and examples.

Example:
```bash
cargo run --example auto_doc_example
```

## Validation and Best Practices

### Generated Nix Expression Validation

**Important:** serde-nixos does NOT validate the generated Nix expressions. You are responsible for ensuring the output is valid.

#### What This Means

1. **Default Values:** The `default = "..."` attribute is inserted directly into the Nix output without syntax checking
2. **Example Values:** The `example = "..."` attribute is not validated
3. **Apply Functions:** The `apply = "..."` attribute can contain arbitrary Nix code
4. **Type Mismatches:** No validation that defaults match the declared type

#### Recommended Validation Workflow

**Option 1: Test with `nix-instantiate` (Quick)**

```bash
# Generate your module
cargo run --example generate_nixos > module.nix

# Validate Nix syntax
nix-instantiate --parse module.nix

# Evaluate (more thorough)
nix-instantiate --eval module.nix
```

**Option 2: NixOS VM Test (Comprehensive)**

The best validation is to test in an actual NixOS VM. See `integration-test/nixos-test.nix` for a complete example:

```nix
import <nixpkgs/nixos/tests/make-test-python.nix> ({ pkgs, ... }: {
  name = "myservice-test";
  
  nodes.machine = { config, pkgs, ... }: {
    imports = [ ./module.nix ];
    
    services.myservice = {
      enable = true;
      port = 8080;
    };
  };
  
  testScript = ''
    machine.wait_for_unit("myservice.service")
    machine.succeed("curl http://localhost:8080")
  '';
})
```

Run with:
```bash
nix-build integration-test/nixos-test.nix
```

**Option 3: Use Nix Flake Checks**

Add to your `flake.nix`:

```nix
checks = {
  # Syntax check
  module-syntax = pkgs.runCommand "check-module" {} ''
    ${pkgs.nix}/bin/nix-instantiate --parse ${./module.nix}
    touch $out
  '';
  
  # VM integration test
  nixos-test = import ./nixos-test.nix { inherit pkgs; };
};
```

Then run:
```bash
nix flake check
```

#### Common Validation Issues

**Issue 1: Unquoted String Defaults**

```rust
// ❌ WRONG - generates invalid Nix
#[nixos(default = "localhost")]
host: String,

// ✅ CORRECT - strings must be quoted in Nix
#[nixos(default = "\"localhost\"")]
host: String,
```

**Issue 2: Type Mismatches**

```rust
// ❌ WRONG - default is string but type is int
#[nixos(default = "\"8080\"")]
port: u16,

// ✅ CORRECT - numeric literals don't need quotes
#[nixos(default = "8080")]
port: u16,
```

**Issue 3: Invalid Nix Syntax in Apply**

```rust
// ❌ WRONG - syntax error in Nix expression
#[nixos(apply = "x: x +")]
value: u32,

// ✅ CORRECT - valid Nix lambda
#[nixos(apply = "x: x + 1")]
value: u32,
```

**Issue 4: Complex Defaults Without Escaping**

```rust
// ❌ WRONG - unescaped quotes
#[nixos(default = "{ "foo" = "bar"; }")]
config: String,

// ✅ CORRECT - use raw strings or escape properly
#[nixos(default = r#"{ foo = "bar"; }"#)]
config: String,
```

### Best Practices

#### 1. Always Validate Generated Output

```bash
# After generating
cargo run --example generate_nixos > module.nix

# Validate syntax
nix-instantiate --parse module.nix

# Test evaluation
nix-instantiate --eval module.nix
```

#### 2. Use Raw Strings for Complex Nix Expressions

```rust
#[nixos(
    default = r#"{ 
        host = "localhost"; 
        port = 8080; 
    }"#,
    description = "Server configuration"
)]
server_config: String,
```

#### 3. Test with NixOS VM Tests

Don't just validate syntax—test the actual behavior in a VM.

#### 4. Use Type-Safe Defaults

```rust
// Prefer Option<T> with no default over potentially invalid defaults
#[nixos(description = "Port (optional)")]
port: Option<u16>,

// Instead of
#[nixos(default = "null", description = "Port")]
port: u16,  // This won't work - u16 can't be null
```

#### 5. Document Expected Nix Types

```rust
/// The server configuration as a Nix attrset
/// Expected format: { host = "..."; port = 123; }
#[nixos(default = r#"{ host = "localhost"; port = 8080; }"#)]
config: HashMap<String, String>,
```

#### 6. Use Examples to Document Valid Values

```rust
#[nixos(
    description = "Log level",
    default = "\"info\"",
    example = "\"debug\""
)]
log_level: String,
```

### Current Limitations

#### Known Issues

1. **No Nix Expression Validation**
   - Generated Nix code is not syntax-checked
   - Invalid defaults/examples will only fail at NixOS evaluation time
   - **Workaround:** Always test with `nix-instantiate` or NixOS VM tests

2. **Limited Enum Support**
   - Enums with data variants (tuple/struct) have basic support
   - May not generate optimal Nix representations
   - **Workaround:** Use simple enums or model as structs with optional fields

3. **HashMap Key Limitations**
   - Non-string keys (integers, enums) may not map well to Nix attrsets
   - Nix attrset keys must be strings
   - **Workaround:** Use `Vec<(K, V)>` or serialize keys to strings

4. **No Generic Type Support**
   - Generic types like `struct Config<T>` are not supported
   - **Workaround:** Use concrete types or generate multiple variants

5. **Union Types Unsupported**
   - Rust `union` types are not supported
 
#### Type Mapping Edge Cases

```rust
// Box<T> - Treated as T
field: Box<String>,  // Generates: types.str

// Rc<T>, Arc<T> - Treated as T  
field: Arc<String>,  // Generates: types.str

// Tuple structs - Generate generic attrs
struct Wrapper(String);  // Generates: types.attrs

// Empty structs - Generate empty submodules
struct Empty {}  // Generates: types.submodule { options = {}; }
```

### Troubleshooting

#### Generated Module Won't Parse

```bash
# Check syntax
nix-instantiate --parse module.nix

# Common causes:
# - Unquoted string defaults
# - Missing semicolons in Nix expressions
# - Unescaped quotes in strings
```

#### Module Evaluates But NixOS Fails

```bash
# Test with nixos-rebuild
sudo nixos-rebuild test -I nixos-config=./test-configuration.nix

# Common causes:
# - Type mismatches between default and type
# - Invalid references in apply functions
# - Missing imports or dependencies
```

#### Generated Types Don't Match Expectations

```rust
// Debug: Print generated output
println!("{}", MyConfig::nixos_type_definition());

// Check each method
println!("Type: {}", MyConfig::nixos_type());
println!("Options: {}", MyConfig::nixos_options());
println!("Name: {}", MyConfig::nixos_type_name());
```

### Testing Your Modules

#### Minimal Test Configuration

Create `test-config.nix`:

```nix
{ config, pkgs, ... }:

{
  imports = [ ./module.nix ];
  
  # Your generated module
  services.myservice = {
    enable = true;
    # Add test values
  };
  
  # Minimal NixOS config for testing
  boot.loader.grub.enable = false;
  fileSystems."/" = { device = "/dev/sda"; };
}
```

Test with:

```bash
nix-instantiate --eval '<nixpkgs/nixos>' -A config.services.myservice \
  -I nixos-config=./test-config.nix
```

#### Integration Test Template

See `integration-test/` for a complete example with:
- Rust code generation
- NixOS module generation  
- VM test with assertions
- Systemd service integration

Run with:
```bash
nix build .#checks.x86_64-linux.nixos-integration
```
