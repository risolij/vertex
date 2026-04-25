{
  description = "Transactions";
  nixConfig.bash-prompt = "\[transaction-shell\]$ ";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        lib = nixpkgs.lib;
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = [
            rust-analyzer
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = ["rust-src"];
            }))
          ];
        };
      }
    );
}
