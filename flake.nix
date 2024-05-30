{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    fenix.inputs.rust-analyzer-src.follows = "";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    nix-github-actions.url = "github:nix-community/nix-github-actions";
    nix-github-actions.inputs.nixpkgs.follows = "nixpkgs";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    , crane
    , nix-github-actions
    , pre-commit-hooks
    , ...
    }:
    flake-utils.lib.eachDefaultSystem
      (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        inherit (pkgs) lib;

        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          # sha256 = pkgs.lib.fakeSha256;
          sha256 = "sha256-UEZYwK60IvJvS7qs+vyVRQeJE6joF9SEcHoNqvIhShw=";
        };

        fenix-pkgs = fenix.packages.${system};

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource (craneLib.path ./.);

        buildInputs = [
          pkgs.openssl
        ] ++ lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.libiconv
        ];

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        commonArgs = {
          pname = "tforge";
          version = "0.1.0";

          strictDeps = true;
          inherit src;
          inherit buildInputs;
          inherit nativeBuildInputs;
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

        workspace-pre-commit-check = pre-commit-hooks.lib.${system}.run {
          inherit src;
          hooks = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
            clippy.enable = true;
            cargo-check.enable = true;
          };
          tools = {
            rustfmt = rustToolchain;
            clippy = rustToolchain;
            cargo = rustToolchain;
          };
        };
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
            inherit (workspace-pre-commit-check) shellHook;

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
            ] ++ workspace-pre-commit-check.enabledPackages;

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
        ]
          self.checks;
      };
    });
}
