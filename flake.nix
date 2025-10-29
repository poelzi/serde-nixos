{
  description = "Generate NixOS type definitions from Rust structures";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , pre-commit-hooks
    ,
    }:
    flake-utils.lib.eachDefaultSystem
      (
        system:
        let
          pkgs = import nixpkgs { inherit system; };

          # Build the test service
          testService = pkgs.rustPlatform.buildRustPackage {
            pname = "serde-nixos-test-service";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildAndTestSubdir = "integration-test";
          };

          # Helper to generate the module
          generateModule = pkgs.writeShellScriptBin "generate-nixos-module" ''
            cd ${./.}
            ${pkgs.cargo}/bin/cargo run --bin generate-module > integration-test/module.nix
            echo "Generated integration-test/module.nix"
          '';

          # CI package - comprehensive Rust checks
          serde-nixos-ci = pkgs.rustPlatform.buildRustPackage {
            pname = "serde-nixos-ci";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            # Build all workspace members and examples
            buildPhase = ''
              runHook preBuild
              echo "Building workspace..."
              cargo build --all --release
              echo "Building examples..."
              cargo build --examples --release
              runHook postBuild
            '';

            # Run comprehensive checks
            checkPhase = ''
              runHook preCheck
              echo "Running tests..."
              cargo test --all --release
              echo "Running clippy..."
              cargo clippy --all-targets --all-features -- -D warnings
              echo "Checking formatting..."
              cargo fmt --all -- --check
              echo "Generating documentation..."
              cargo doc --all --no-deps --document-private-items
              runHook postCheck
            '';

            # Install binaries
            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin
              # Copy built binaries if they exist
              cp target/release/examples/advanced_features $out/bin/ 2>/dev/null || true
              cp target/release/test-service $out/bin/ 2>/dev/null || true
              cp target/release/generate-module $out/bin/ 2>/dev/null || true
              runHook postInstall
            '';

            doCheck = true;

            meta = {
              mainProgram = "advanced_features";
            };

            nativeBuildInputs = [ pkgs.rustfmt pkgs.clippy pkgs.coreutils ];
          };

          # Pre-commit hooks (formatting only - comprehensive checks in rust-ci package)
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              # Rust formatting
              rustfmt = {
                enable = true;
              };
              clippy = {
                enable = true;
              };

              # Nix formatting
              nixpkgs-fmt = {
                enable = true;
              };
            };
          };

        in
        {
          packages = {
            inherit testService generateModule;
            ci = serde-nixos-ci;
            default = testService;
          };

          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              rustc
              rust-analyzer
              clippy
              rustfmt
              cargo-watch
              nixpkgs-fmt
              generateModule
            ];

            shellHook = pre-commit-check.shellHook + ''
              echo "🦀 serde-nixos development environment"
              echo ""
              echo "Commands:"
              echo "  cargo build                     - Build the library"
              echo "  cargo test                      - Run Rust tests"
              echo "  cargo run --example X           - Run example"
              echo "  nix flake check                 - Run all checks including NixOS VM test"
              echo "  nix build .#testService         - Build integration test service"
              echo "  generate-nixos-module           - Regenerate NixOS module"
              echo ""
              echo "Pre-commit hooks are installed. They will run automatically on git commit."
            '';
          };

          checks = {
            # Comprehensive Rust CI check (tests, clippy, fmt, build, examples)
            rust-ci = serde-nixos-ci;

            # Nix formatting check
            nix-fmt =
              pkgs.runCommand "nix-fmt-check"
                {
                  buildInputs = [ pkgs.nixpkgs-fmt ];
                }
                ''
                  echo "Checking Nix formatting..."
                  nixpkgs-fmt --check ${./flake.nix}
                  nixpkgs-fmt --check ${./integration-test/nixos-test.nix}
                  touch $out
                '';

            # Build the test service (includes running tests in check phase)
            test-service-builds = testService;

            # NixOS VM integration test
            nixos-integration = import ./integration-test/nixos-test.nix {
              inherit pkgs system testService;
              serde-nixos-src = ./.;
            };
          };

          # Run the NixOS test interactively
          apps.nixos-test = {
            type = "app";
            program = toString (
              pkgs.writeShellScript "run-nixos-test" ''
                echo "Running NixOS integration test..."
                ${pkgs.nix}/bin/nix build .#checks.${system}.nixos-integration --print-build-logs
                echo "✓ Integration test passed!"
              ''
            );
          };

          # Formatter for `nix fmt`
          formatter = pkgs.writeShellScriptBin "format-all" ''
            set -e
            echo "Formatting Rust code..."
            ${pkgs.cargo}/bin/cargo fmt --all
            echo "Formatting Nix files..."
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt flake.nix integration-test/*.nix
            echo "✓ All files formatted"
          '';
        }
      )
    // {
      # Expose the NixOS module
      nixosModules.test-service = import ./integration-test/module.nix;
    };
}
