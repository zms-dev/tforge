{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
    nix-github-actions.url = "github:nix-community/nix-github-actions";
    nix-github-actions.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    naersk,
    nix-github-actions,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { 
          inherit system;
           overlays = [(import rust-overlay)];
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk' = pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        };
      in 
      {
        devShells.default = pkgs.mkShell {
          name = "shell";

          nativeBuildInputs = with pkgs; [
            gcc
            openssl.dev
            pkg-config
            rust
            cargo-expand
            cargo-watch
            nil
          ];

          RUST_PATH = "${rust}";
          RUST_DOC_PATH = "${rust}/share/doc/rust/html/std/index.html";
        };

        packages.default = naersk'.buildPackage {
            src = ./.;
        };

        checks.default = naersk'.buildPackage {
            src = ./.;
            mode = "test";
        };
      })
      // ({
        githubActions = nix-github-actions.lib.mkGithubMatrix {
          checks = nixpkgs.lib.getAttrs [ "x86_64-linux" "x86_64-darwin" ] self.checks;
        };
      });
}