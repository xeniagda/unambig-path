{
  description = "Unambiguous path";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (sys:
    let pkgs = nixpkgs.legacyPackages.${sys};
        unambig-path = pkgs.rustPlatform.buildRustPackage {
          name = "unambig-path" ;
          src = ./. ;
          cargoLock = { lockFile = ./Cargo.lock; };
        };
    in {
      packages.default = unambig-path;
    }
  );
}
