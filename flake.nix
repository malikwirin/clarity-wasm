{
  description = "Clarity to WebAssembly compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        buildRsPatch = ./nix/patches/build-rs.patch;

        wasm_generatorsPatch = ./nix/patches/wasm-generators.patch;

        commonArgs = {
          pname = "clarity-wasm";
          version = "0.1.0";
          
          src = ./.;

          postPatch = ''
            patch -p1 < ${buildRsPatch}
            patch -p1 < ${wasm_generatorsPatch}
          '';

          buildInputs = with pkgs; [
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        clar2wasm = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "clar2wasm";
          cargoBuildCommand = "cargo build --release --package clar2wasm";
        });

        clarity-wasm-tools = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "clarity-wasm-tools";
          cargoBuildCommand = "cargo build --release --package clarity-wasm-tools";
        });

        clar2wasm-tests = craneLib.cargoTest (commonArgs // {
          inherit cargoArtifacts;
          pname = "clar2wasm-tests";
        });
      in
      {
        packages = {
          inherit clar2wasm clarity-wasm-tools;
          default = clar2wasm;
        };

        checks = {
          inherit clar2wasm-tests;
        };

        devShells.default = craneLib.devShell {
          inputsFrom = [ clar2wasm ];
          packages = with pkgs; [
            rustToolchain
            rust-analyzer
          ];
        };
      }
    );
}
