{
  description = "Plugin adding uuid to sqlite";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          name = "libsqlite_uuid";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = true;

          buildInputs = with pkgs; [
            sqlite-interactive
          ] ++ lib.optionals stdenv.isDarwin [
            libiconv
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.Foundation
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };
        devenv.shells.default = {
          name = "sqlite-uuid";

          # https://devenv.sh/reference/options/
          # on mac sqlite is not compiled with 'load' support
          packages = with pkgs; [ sqlite-interactive ];

        };
      };
    };
}
