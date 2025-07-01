{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        oil = pkgs.rustPlatform.buildRustPackage {
          pname = "oil";
          version = "0.1.0"; # set your version
          doCheck = false; # Disable tests during nix builds. These do not play nicely with compiletest_rs

          # Path to your project source code (relative to flake root)
          # Assuming your project is in the same directory as this flake file:
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # Optionally, override cargo build flags, tests, etc.
          # cargoBuildFlags = ["--release"];
          # buildPhase = "cargo build --release";

          # You can add other settings like meta or doCheck (tests) here
        };
      in {
        # Shell dependencies
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];
        };

        packages = {
          default = oil;
        };
      }
    );
}
