{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { 
          inherit system;
           overlays = [(import rust-overlay)];
        };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk = pkgs.callPackage naersk {
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

        # defaultPackage = naersk.buildPackage ./.;
      });
}