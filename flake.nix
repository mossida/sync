{
  description = "virtual environments";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.flake-compat = {
    url = "github:edolstra/flake-compat";
    flake = false;
  };

  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, flake-utils, devshell, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system: {
      devShells.default = let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            devshell.overlays.default
            rust-overlay.overlays.default
            (final: prev: {
              rustToolchain = final.rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
              };
            })
          ];
        };
      in pkgs.devshell.mkShell {
        packages = with pkgs; [rustToolchain];
        imports = [ (pkgs.devshell.importTOML ./devshell.toml) ];
      };
    });
}