{
  description = "A webserver for use with flake-watcher";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      rec {
        name = "flake-server";
        packages.flake-server = import ./default.nix { pkgs = nixpkgs.legacyPackages.${system}; };
        packages.default = packages.flake-server;
      }
    );
}
