# This file has been generated by node2nix 1.11.1. Do not edit!

{ pkgs ? import <nixpkgs> { } }:

with pkgs;
rustPlatform.buildRustPackage rec {
  pname = "flake-server";
  version = "0.1.0";

  src = ./.;

  nativeBuildInputs = [
    rustfmt
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";


  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with stdenv.lib; {
    description = "A auto-reloading webserver that hosts paths from stdin";
  };
}
