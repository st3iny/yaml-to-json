{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-26.05";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        craneLib = crane.mkLib pkgs;
        crateName = craneLib.crateNameFromCargoToml {
          cargoToml = ./yaml-to-json/Cargo.toml;
        };

        nightlyCraneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.selectLatestNightlyWith (
            toolchain:
            toolchain.default.override {
              extensions = [
                "rust-analyzer"
                "rust-src"
              ];
            }
          )
        );
      in
      {
        packages.default = craneLib.buildPackage {
          inherit (crateName) pname version;
          src = craneLib.cleanCargoSource ./.;
        };
        devShells.default = nightlyCraneLib.devShell {
          packages = [
          ];
        };
      }
    );
}
