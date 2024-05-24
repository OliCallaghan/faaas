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
        python = pkgs.python311;
        pythonPackages = python.pkgs;
      in
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.bashInteractive ];
          buildInputs = with pkgs; [
            clang-tools
            libcxxStdenv

            cmake
            cmake-format
			cmake-language-server

            racket

            corepack_20
            nodejs_20
            rustup
            cargo-watch
            wasmtime
            minio
            texliveFull
            graphviz
            pythonPackages.venvShellHook
            pkgs.nodePackages.pyright
            pkgs.poetry
            pythonPackages.setuptools
            pythonPackages.wheel
          ] ++ lib.optional stdenv.isDarwin libiconv ++ lib.optional stdenv.isDarwin [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          venvDir = ".venv";
          src = null;

          packages = [
            (python.withPackages (python-pkgs: [
                python-pkgs.packaging
            ]))
          ];

          shellHook = "pnpm install --frozen-lockfile";

          VIRTUAL_ENV_DISABLE_PROMPT = true;

          postVenv = ''
            unset SOURCE_DATE_EPOCH
          '';
          postShellHook = ''
            unset SOURCE_DATE_EPOCH
            unset LD_PRELOAD

            PYTHONPATH=$PWD/$venvDir/${python.sitePackages}:$PYTHONPATH
          '';
        };
      }
    );
}
