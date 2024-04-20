{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let 
        overlays = [
          fenix.overlays.default
        ];
        pkgs = import nixpkgs { 
          inherit system overlays;
        };
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          # sha256 = nixpkgs.lib.fakeSha256;
          sha256 = "sha256-o+ymjPUfTAzCiFp6qdcPpjus293CYYNvW+mP9TIPaT0=";
        };
      in 
      {
        devShells.default = pkgs.mkShell {
          name = "shell";

          nativeBuildInputs = with pkgs; [
            openssl.dev
            pkg-config
            toolchain
          ];

          buildInputs = with pkgs; [
            toolchain
          ];
        };
      });
}