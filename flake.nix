{
  description = "Dev environment for NixOS";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain: toolchain.default.override { 
            extensions = [ "rust-analyzer" "rust-src" ];
            targets = [];
          }
        );
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          buildInputs = [
            mold
            openssl
            pkg-config
            llvmPackages.bintools
            clang
            rust
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
