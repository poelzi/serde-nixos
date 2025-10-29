# Auto-generated NixOS module for test-service
# Generated from Rust types using serde-nixos

{ config
, lib
, pkgs
, ...
}:

with lib;

let
  cfg = config.services.test-service;

  # Type definitions
  # NixOS type definition for ServerConfig
  serverConfigType = types.submodule {
    options = {
      enable = lib.mkOption {
        type = types.bool;
        description = "Enable the server";
        default = false;
      };

      port = lib.mkOption {
        type = types.int;
        description = "Server port";
        default = 8080;
      };

      bind_address = lib.mkOption {
        type = types.str;
        description = "Server bind address";
        default = "127.0.0.1";
      };

      max_connections = lib.mkOption {
        type = types.int;
        description = "Maximum connections";
        default = 100;
      };

    };
  };

  # NixOS type definition for DatabaseConfig
  databaseConfigType = types.submodule {
    options = {
      host = lib.mkOption {
        type = types.str;
        description = "Database host";
        default = "localhost";
      };

      port = lib.mkOption {
        type = types.int;
        description = "Database port";
        default = 5432;
      };

      database = lib.mkOption {
        type = types.str;
        description = "Database name";
      };

      ssl = lib.mkOption {
        type = types.bool;
        description = "Enable SSL";
        default = false;
      };

    };
  };

  # Generate config file
  configFile = pkgs.writeText "test-service-config.json" (
    builtins.toJSON {
      service_name = cfg.service_name;
      server = {
        enable = cfg.server.enable;
        port = cfg.server.port;
        bind_address = cfg.server.bind_address;
        max_connections = cfg.server.max_connections;
      };
      database = {
        host = cfg.database.host;
        port = cfg.database.port;
        database = cfg.database.database;
        ssl = cfg.database.ssl;
      };
      debug = cfg.debug;
      log_level = cfg.log_level;
    }
  );
in
{
  options.services.test-service = {
    enable = mkEnableOption "test service";

    package = mkOption {
      type = types.package;
      default = pkgs.serde-nixos-test-service or pkgs.hello;
      description = "The test-service package to use";
    };

    service_name = mkOption {
      type = types.str;
      default = "test-service";
      description = "Service name";
    };

    server = mkOption {
      type = serverConfigType;
      description = "Server configuration";
    };

    database = mkOption {
      type = databaseConfigType;
      description = "Database configuration";
    };

    debug = mkOption {
      type = types.bool;
      default = false;
      description = "Enable debug logging";
    };

    log_level = mkOption {
      type = types.str;
      default = "info";
      description = "Log level";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.test-service = {
      description = "Test Service for serde-nixos integration testing";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "oneshot";
        ExecStart = "${cfg.package}/bin/test-service ${configFile}";
        RemainAfterExit = true;
      };
    };
  };
}
