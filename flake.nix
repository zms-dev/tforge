{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    nix-github-actions.url = "github:nix-community/nix-github-actions";
    nix-github-actions.inputs.nixpkgs.follows = "nixpkgs";
    advisory-db.url = "github:rustsec/advisory-db";
    advisory-db.flake = false;
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    nix-github-actions,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { 
          inherit system;
          overlays = [(import rust-overlay)];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource (craneLib.path ./.);

        commonArgs = {
          pname = "tforge";
          version = "0.1.0";

          inherit src;
          strictDeps = true;

          buildInputs = [
            pkgs.openssl
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          nativeBuildInputs = [
            pkgs.pkg-config
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        workspace = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        workspace-clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
        });

        workspace-cargo-doc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
        });

        workspace-cargo-nextest = craneLib.cargoNextest (commonArgs // {
          inherit cargoArtifacts;
          partitions = 1;
          partitionType = "count";
        });

        workspace-cargo-llvm-cov = craneLib.cargoLlvmCov (commonArgs // {
          inherit cargoArtifacts;
        });
      in 
      {
        packages = {
          default = workspace;
          cargo-llvm-cov = workspace-cargo-llvm-cov;
        };

        checks = {
          inherit workspace;

          clippy = workspace-clippy;
          cargo-doc = workspace-cargo-doc;
          cargo-nextest = workspace-cargo-nextest;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = workspace;
        };

        devShells = {
          default = craneLib.devShell {
            checks = self.checks.${system};

            packages = with pkgs; [
              gcc
              cargo-watch
              cargo-deny
              cargo-audit
              cargo-update
              cargo-edit
              cargo-outdated
              cargo-license
              cargo-tarpaulin
              cargo-nextest
              cargo-spellcheck
              cargo-modules
              cargo-bloat
              cargo-expand
              cargo-llvm-cov
              nil
            ];

            RUST_PATH = "${rustToolchain}";
            RUST_DOC_PATH = "${rustToolchain}/share/doc/rust/html/std/index.html";
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";

            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        };
      }) // ({
        githubActions = nix-github-actions.lib.mkGithubMatrix {
          checks = nixpkgs.lib.getAttrs [
            flake-utils.lib.system.x86_64-linux
            flake-utils.lib.system.x86_64-darwin
          ] self.checks;
        };
      });
}