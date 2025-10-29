{ pkgs ? import <nixpkgs> { }
, system ? builtins.currentSystem
, testService ? null
, serde-nixos-src ? ../.
,
}:

let
  # Build the test service if not provided
  actualTestService =
    if testService != null then
      testService
    else
      pkgs.rustPlatform.buildRustPackage {
        pname = "serde-nixos-test-service";
        version = "0.1.0";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };

  # Expected configuration that we'll set in NixOS
  expectedConfig = pkgs.writeText "expected-config.json" (
    builtins.toJSON {
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
    }
  );

in
pkgs.nixosTest {
  name = "serde-nixos-integration";

  nodes.machine =
    { config
    , pkgs
    , lib
    , ...
    }:
    {
      # Import our generated module
      imports = [ ./module.nix ];

      # Make the test service package available
      nixpkgs.overlays = [
        (self: super: {
          serde-nixos-test-service = actualTestService;
        })
      ];

      # Configure the service using the generated NixOS module
      services.test-service = {
        enable = true;
        package = pkgs.serde-nixos-test-service;
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
    };

  testScript = ''
    start_all()
    machine.wait_for_unit("test-service.service")

    # The service runs once and validates the config
    # Check that it succeeded
    machine.succeed("systemctl status test-service.service")

    # Extract the generated config file and validate it
    config_file = machine.succeed(
      "systemctl cat test-service.service | grep ExecStart | awk '{print $NF}'"
    ).strip()

    print(f"Config file: {config_file}")

    # Validate that the config matches our expected values
    machine.succeed(
      f"${actualTestService}/bin/test-service --validate {config_file} ${expectedConfig}"
    )

    # Check the journal output
    output = machine.succeed("journalctl -u test-service.service")
    print("Service output:")
    print(output)

    # Verify specific values in the output
    machine.succeed("journalctl -u test-service.service | grep 'Service Name: integration-test'")
    machine.succeed("journalctl -u test-service.service | grep 'Address: 0.0.0.0:3000'")
    machine.succeed("journalctl -u test-service.service | grep 'Host: db.example.com'")
    machine.succeed("journalctl -u test-service.service | grep 'Database: testdb'")
    machine.succeed("journalctl -u test-service.service | grep 'Debug: true'")
    machine.succeed("journalctl -u test-service.service | grep 'Level: debug'")
    machine.succeed("journalctl -u test-service.service | grep 'Configuration loaded successfully'")
  '';
}
