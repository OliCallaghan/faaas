{
  description = "FaaAS Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            corepack_20
            nodejs_20
            rustup
            texliveMedium
          ];

          shellHook = "pnpm install --frozen-lockfile";
        };
      }
    );
}
