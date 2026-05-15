{
  description = "Shared Bevy+egui WASM crates (bevy-core, ui-theme)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        # Expose the raw source for downstream Nix builds (crane merged-src).
        packages.src = pkgs.runCommand "bevy-wasm-core-src" { } ''
          cp -r ${pkgs.lib.cleanSource ./.}/. $out
        '';

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.wasm-pack
            pkgs.wasm-bindgen-cli
          ];
        };
      }
    );
}
