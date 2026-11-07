{ pkgs ? import <nixpkgs> {} }:
  pkgs.rustPlatform.buildRustPackage {
    name = "unambig-path" ;
    src = ./. ;
    cargoLock = { lockFile = ./Cargo.lock; };
  }
